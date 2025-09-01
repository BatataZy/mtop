use crate::io::read;

#[derive(Clone, Debug)]
pub struct Ram {
    pub used: usize,
    pub allocated: usize,
    pub total: usize,
}
#[derive(Clone, Debug)]
pub struct Swap {
    pub allocated: usize,
    pub total: usize,
}

#[derive(Clone, Debug)]
pub struct Memory {
    pub ram: Ram,
    pub swap: Swap,
}

impl Memory {
    pub fn new() -> Self {
        let memory = read("/proc/meminfo", 0, 0)
            .split('\n')
            .filter(|line| line.starts_with("Mem") || line.starts_with("Swap"))
            .map(|line| {
                line.rsplit(' ')
                    .nth(1)
                    .expect("there is an element on index 1")
                    .parse::<usize>()
                    .expect("fully numeric string")
                    / 1024
            })
            .collect::<Vec<usize>>();

        Self {
            ram: Ram {
                used: memory[0] - memory[2],
                allocated: memory[0] - memory[1],
                total: memory[0],
            },

            swap: Swap {
                allocated: memory[4] - memory[5],
                total: memory[4],
            },
        }
    }

    pub fn update(&mut self) {
        let memory = read("/proc/meminfo", 0, 0)
            .split('\n')
            .filter(|line| line.starts_with("Mem") || line.starts_with("Swap"))
            .map(|line| {
                line.rsplit(' ')
                    .nth(1)
                    .expect("there is an element on index 1")
                    .parse::<usize>()
                    .expect("fully numeric string")
                    / 1024
            })
            .collect::<Vec<usize>>();

        self.ram.used = self.ram.total - memory[2];

        self.ram.allocated = self.ram.total - memory[1];

        self.swap.allocated = self.swap.total - memory[4];
    }
}
