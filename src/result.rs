use crate::{cpu::Cpu, gpu::Gpu, network::Network, ram::Memory};
use num_traits::{Num, NumCast};

static NULL: &str = " ";

fn prettify(x: &str, size: usize, blank: &str, prefix: &str) -> String {
    blank.to_owned().repeat(size - x.len()) + x + prefix
}

fn median<T: Num + NumCast + Copy + Ord>(mut list: Vec<T>) -> T {
    list.sort();

    match list.len() % 2 {
        0 => {
            (list[list.len() / 2] + list[list.len() / 2 - 1])
                / T::from(2).expect("'T' is a number type and should therefore have a number 2")
        }
        _ => list[list.len() / 2],
    }
}

#[derive(serde::Serialize)]
struct Clock {
    median: String,
    arithmetic_mean: String,
    max: String,
    min: String,
    values: Vec<String>,
}
impl Clock {
    fn prettify(self) -> Self {
        Self {
            median: prettify(&self.median, 4, NULL, "MHz"),
            arithmetic_mean: prettify(&self.arithmetic_mean, 4, NULL, "MHz"),
            max: prettify(&self.max, 4, NULL, "MHz"),
            min: prettify(&self.min, 4, NULL, "MHz"),
            values: self
                .values
                .iter()
                .map(|x| prettify(x, 4, NULL, "MHz"))
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
struct Util {
    median: String,
    arithmetic_mean: String,
    max: String,
    min: String,
    values: Vec<String>,
}
impl Util {
    fn prettify(self) -> Self {
        Self {
            median: prettify(&self.median, 2, "0", "%"),
            arithmetic_mean: prettify(&self.arithmetic_mean, 2, "0", "%"),
            max: prettify(&self.max, 2, "0", "%"),
            min: prettify(&self.min, 2, "0", "%"),
            values: self
                .values
                .iter()
                .map(|x| prettify(x, 2, "0", "%"))
                .collect(),
        }
    }
}

#[derive(serde::Serialize)]
struct CpuResult {
    pub clock: Clock,
    pub util: Util,
    pub temp: String,
}
impl CpuResult {
    pub fn new(cpu: &Cpu) -> Self {
        let clocks = cpu.clock.iter().map(|x| x.average);
        let utils = cpu.util.iter();

        Self {
            clock: Clock {
                median: median(clocks.clone().collect::<Vec<u16>>()).to_string(),
                arithmetic_mean: (clocks.clone().sum::<u16>() / cpu.threads).to_string(),
                max: clocks
                    .clone()
                    .max()
                    .expect("iterator is not empty")
                    .to_string(),
                min: clocks
                    .clone()
                    .min()
                    .expect("iterator is not empty")
                    .to_string(),
                values: clocks
                    .clone()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>(),
            },
            util: Util {
                median: median(cpu.util.clone()).min(99).to_string(),
                arithmetic_mean: (utils.clone().copied().sum::<u16>() / cpu.threads)
                    .min(99)
                    .to_string(),
                max: utils
                    .clone()
                    .max()
                    .expect("iterator is not empty")
                    .min(&99)
                    .to_string(),
                min: utils
                    .clone()
                    .min()
                    .expect("iterator is not empty")
                    .min(&99)
                    .to_string(),
                values: cpu
                    .util
                    .iter()
                    .map(|x| x.min(&99).to_string())
                    .collect::<Vec<String>>(),
            },
            temp: format!("{:.1}", cpu.temp),
        }
    }

    fn prettify(self) -> Self {
        Self {
            clock: self.clock.prettify(),
            util: self.util.prettify(),
            temp: prettify(&self.temp, 4, NULL, "°C"),
        }
    }
}

#[derive(serde::Serialize)]
struct GpuResult {
    pub clock: String,
    pub util: String,
    pub temp: String,
    pub vram: Swap,
}
impl GpuResult {
    pub fn new(gpu: &Gpu) -> Self {
        Self {
            clock: gpu.clock.average.to_string(),
            util: gpu.util.average.min(99).to_string(),
            temp: format!("{:.1}", gpu.temp),
            vram: Swap {
                allocated: gpu.memory.used.to_string(),
                total: gpu.memory.total.to_string(),
            },
        }
    }

    fn prettify(self) -> Self {
        Self {
            clock: prettify(&self.clock, 4, NULL, "MHz"),
            util: prettify(&self.util, 2, "0", "%"),
            temp: prettify(&self.temp, 4, NULL, "°C"),
            vram: self.vram.prettify(),
        }
    }
}

#[derive(serde::Serialize)]
struct Ram {
    used: String,
    allocated: String,
    total: String,
}
impl Ram {
    fn prettify(self) -> Self {
        Self {
            used: prettify(&self.used, self.total.len(), NULL, ""),
            allocated: prettify(&self.allocated, self.total.len(), NULL, ""),
            total: self.total + "",
        }
    }
}

#[derive(serde::Serialize)]
struct Swap {
    allocated: String,
    total: String,
}
impl Swap {
    fn prettify(self) -> Self {
        Self {
            allocated: prettify(&self.allocated, self.total.len(), NULL, ""),
            total: self.total + "",
        }
    }
}

#[derive(serde::Serialize)]
struct MemoryResult {
    ram: Ram,
    swap: Swap,
}
impl MemoryResult {
    fn new(mem: &Memory) -> Self {
        Self {
            ram: Ram {
                used: mem.ram.used.to_string(),
                allocated: mem.ram.allocated.to_string(),
                total: mem.ram.total.to_string(),
            },
            swap: Swap {
                allocated: mem.swap.allocated.to_string(),
                total: mem.swap.total.to_string(),
            },
        }
    }

    fn prettify(self) -> Self {
        Self {
            ram: self.ram.prettify(),
            swap: self.swap.prettify(),
        }
    }
}

#[derive(serde::Serialize)]
struct DiskResult {}
impl DiskResult {
    const fn prettify(self) -> Self {
        self
    }
}

#[derive(serde::Serialize)]
struct Adapter {
    name: String,
    interface: String,
    vendor: String,
    connection: String,
    local_ip: String,
    public_ip: String,
}
impl Adapter {
    fn prettify(self) -> Self {
        Self {
            name: self.name,
            interface: self.interface,
            vendor: self.vendor,
            connection: self.connection,
            local_ip: self
                .local_ip
                .split('.')
                .map(|s| prettify(s, 3, NULL, ""))
                .collect::<Vec<String>>()
                .join("."),
            public_ip: self
                .public_ip
                .split('.')
                .map(|s| prettify(s, 3, NULL, ""))
                .collect::<Vec<String>>()
                .join("."),
        }
    }
}

#[derive(serde::Serialize)]
struct NetworkResult {
    adapter: Adapter,
}
impl NetworkResult {
    fn new(net: &Network) -> Self {
        Self {
            adapter: Adapter {
                name: net.adapter.name.clone(),
                interface: net.adapter.interface.clone(),
                vendor: net.adapter.vendor.clone(),
                connection: net.adapter.connection.clone(),
                local_ip: net.adapter.local_ip.to_string(),
                public_ip: net.public_ip.to_string(),
            },
        }
    }

    fn prettify(self) -> Self {
        Self {
            adapter: self.adapter.prettify(),
        }
    }
}

#[derive(serde::Serialize)]
pub struct Result {
    cpu: CpuResult,
    gpu: GpuResult,
    memory: MemoryResult,
    disk: DiskResult,
    network: NetworkResult,
}
impl Result {
    pub fn new(cpu: &Cpu, gpu: &Gpu, mem: &Memory, net: &Network) -> Self {
        Self {
            cpu: CpuResult::new(cpu),
            gpu: GpuResult::new(gpu),
            memory: MemoryResult::new(mem),
            disk: DiskResult {},
            network: NetworkResult::new(net),
        }
    }

    pub fn prettify(self) -> Self {
        Self {
            cpu: self.cpu.prettify(),
            gpu: self.gpu.prettify(),
            memory: self.memory.prettify(),
            disk: self.disk.prettify(),
            network: self.network.prettify(),
        }
    }
}
