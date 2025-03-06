use std::{fs, io::Read, path, process::{self}, thread, time};
use home::home_dir;
use serde_json;
use once_cell::sync::Lazy;


mod profiler; use profiler::Profiler;
mod result; use result::Result;
mod cpu; use cpu::Cpu;
mod gpu; use gpu::Gpu;
mod ram; use ram::Memory;
mod network; use network::Network;
mod unit_types;


static BUFFER: usize = 4000;
static STEP: u64 = (BUFFER as f32/ITER as f32 *1000.) as u64;
static ITER: usize = 40;
static UID: Lazy<String> = Lazy::new(||
    String::from_utf8(
    process::Command::new("id").arg("-u")
    .output().unwrap().stdout.split(|x| x == &10).nth(0).unwrap().to_owned()
).unwrap());
static PROFILING: bool = false;


#[tokio::main]
async fn main() {

    fs::create_dir(["/run", "user", &*UID, "tavtop"].iter().collect::<path::PathBuf>()).ok();

    let mut buf: String = String::with_capacity(4096);

    let write_path = home_dir().unwrap().join(".data");

    let mut cpu = Cpu::new(&mut buf);
    let mut gpu = Gpu::new(&mut buf);
    let mut mem = Memory::new(&mut buf);
    let mut net = Network::new();

    let mut profiler = Profiler::new();

    //run("ping google.com", "ping_test").await;

    loop {

        let delta = profiler.update(|| {

            update_all(&mut buf, &mut cpu, &mut gpu, &mut mem, &mut net).await;

            fs::write(&write_path.join("result").to_str().unwrap().to_owned(),serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem, &net)).unwrap()).ok();

            fs::write(&write_path.join("result_pretty").to_str().unwrap().to_owned(),serde_json::to_string_pretty(&Result::new(&cpu, &gpu, &mem, &net).prettify()).unwrap()).ok();
        
        });

        thread::sleep(time::Duration::from_micros((STEP - delta).max(0)));

    }
}


async fn update_all(buf: &mut String, cpu: &mut Cpu, gpu: &mut Gpu, mem: &mut Memory, net: &mut Network) {

    cpu.update(buf);
    gpu.update(buf);
    mem.update(buf);
    net.ip_update();
}


pub fn read(path: &str, buf: &mut String, start: usize, end: usize) -> String {

    let mut file = fs::File::open(path).unwrap();

    buf.clear();

    let read = file.read_to_string(buf);

    return match read {
        Ok(_) => {
            buf[start..buf.len()-1-end].to_owned()
        },
        Err(_) => ("0").to_owned(),
    };
}


async fn run(command: &str, name: &str) -> std::io::Result<()> {

    let file = fs::File::create(["/run", "user", &*UID, env!("CARGO_PKG_NAME"), name].iter().collect::<path::PathBuf>()).unwrap();

    let command = command.split(' ').collect::<Vec<&str>>();

    process::Command::new(command[0])
        .args(command.iter().skip(1))
        .stdout(file)
        .spawn()?;

    Ok(())
}