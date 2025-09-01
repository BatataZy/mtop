#![feature(trim_prefix_suffix)]

use home::home_dir;
use once_cell::sync::Lazy;
use std::{
    ffi::OsStr,
    fs,
    io::Read,
    path::{self, PathBuf},
    process::{self},
    thread, time,
};

mod profiler;
use profiler::Profiler;
mod result;
use result::Result;
mod cpu;
use cpu::Cpu;
mod gpu;
use gpu::Gpu;
mod ram;
use ram::Memory;
mod network;
use network::Network;
mod unit_types;

static BUFFER: usize = 4000;
static STEP: u64 = (BUFFER as f32 / ITER as f32 * 1000.) as u64;
static ITER: usize = 40;
static UID: Lazy<String> = Lazy::new(|| {
    String::from_utf8(
        process::Command::new("id")
            .arg("-u")
            .output()
            .unwrap()
            .stdout
            .split(|x| x == &10)
            .nth(0)
            .unwrap()
            .to_owned(),
    )
    .unwrap()
});
static PROFILING: bool = false;

fn main() {
    fs::create_dir(
        ["/run", "user", &*UID, "tavtop"]
            .iter()
            .collect::<path::PathBuf>(),
    )
    .ok();

    let mut buf: String = String::with_capacity(8192);

    let write_path = home_dir()
        .expect("Couldn't access home directory.")
        .join(".data");

    let mut cpu = Cpu::new(&mut buf);
    let mut gpu = Gpu::new(&mut buf);
    let mut mem = Memory::new(&mut buf);
    let mut net = Network::new();

    let mut profiler = Profiler::new();

    loop {
        let delta = profiler.update(|| {
            update_all(&mut buf, &mut cpu, &mut gpu, &mut mem, &mut net);

            fs::write(
                &write_path.join("result").to_str().unwrap().to_owned(),
                serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem, &net)).unwrap(),
            )
            .ok();

            fs::write(
                &write_path
                    .join("result_pretty")
                    .to_str()
                    .unwrap()
                    .to_owned(),
                serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem, &net).prettify())
                    .unwrap(),
            )
            .ok();
        });

        thread::sleep(time::Duration::from_micros(STEP - delta.min(STEP)));
    }
}

fn update_all(buf: &mut String, cpu: &mut Cpu, gpu: &mut Gpu, mem: &mut Memory, net: &mut Network) {
    cpu.update(buf);
    gpu.update(buf);
    mem.update(buf);
    net.ip_update();
}

pub fn read(path: &str, buf: &mut String, start: usize, end: usize) -> String {
    let mut file = fs::File::open(path).expect("Couldn't open the file");

    buf.clear();

    let read = file.read_to_string(buf);

    match read {
        Ok(_) => buf[start..buf.len() - 1 - end].to_owned(),
        Err(_) => ("0").to_owned(),
    }
}

pub fn file_open(path: &PathBuf, truncate: bool) -> fs::File {
    // let file = if path.exists() {
    let Ok(file) = fs::File::options().truncate(truncate).open(path) else {
        todo!()
    };

    file
}

/// # Panics
///
/// Will panic if the given command fails to run.
pub fn run(command: &str) -> std::process::Output {
    let arguments: Vec<&OsStr> = command.split(' ').map(OsStr::new).collect();
    process::Command::new(arguments[0])
        .args(arguments.iter().skip(1))
        .output()
        .unwrap_or_else(|_| panic!("Failed to run command: '{command}'"))
}
