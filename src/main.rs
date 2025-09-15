#![feature(trim_prefix_suffix, iter_next_chunk)]

use home::home_dir;
use std::{ffi::OsStr, fs, path, process, sync::LazyLock, thread, time};

mod io;
mod unit_types;

mod cpu;
use cpu::Cpu;
mod gpu;
use gpu::Gpu;
mod ram;
use ram::Memory;
mod network;
use network::Network;

mod profiler;
use profiler::Profiler;
mod result;
use result::Result;

static BUFFER: u16 = 4;
static STEP: u64 = BUFFER as u64 * 1_000_000 / ITER as u64;
static ITER: u16 = 40;
static UID: LazyLock<String> = LazyLock::new(|| {
    String::from_utf8_lossy(&run("id -u").stdout)
        .trim_suffix("\n")
        .to_string()
});
static WRITE_PATH: LazyLock<String> = LazyLock::new(|| {
    home_dir()
        .expect("Home directory is reachable")
        .join(".data")
        .to_string_lossy()
        .to_string()
});

static PROFILING: bool = true;

fn main() {
    fs::create_dir(
        ["/run", "user", &*UID, "tavtop"]
            .iter()
            .collect::<path::PathBuf>(),
    )
    .ok();

    let mut cpu = Cpu::new();
    let mut gpu = Gpu::new();
    let mut mem = Memory::new();
    let mut net = Network::new();

    let mut profiler = Profiler::new();

    loop {
        let delta = profiler.update(|| {
            update_all(&mut cpu, &mut gpu, &mut mem, &mut net);

            fs::write(
                WRITE_PATH.clone() + "/result",
                serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem, &net))
                    .expect("valid json"),
            )
            .ok();

            fs::write(
                WRITE_PATH.clone() + "/result_pretty",
                serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem, &net).prettify())
                    .expect("valid json"),
            )
            .ok();
        });

        thread::sleep(time::Duration::from_micros(STEP - delta.min(STEP)));
    }
}

fn update_all(cpu: &mut Cpu, gpu: &mut Gpu, mem: &mut Memory, net: &mut Network) {
    cpu.update();
    gpu.update();
    mem.update();
    net.update();
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
