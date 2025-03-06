use crate::read;

#[derive(Clone, Debug)]
pub struct Ram {
    pub used: usize,
    pub allocated: usize,
    pub total: usize
}
#[derive(Clone, Debug)]
pub struct Swap {
    pub allocated: usize,
    pub total: usize
}


#[derive(Clone, Debug)]
pub struct Memory {
    pub ram: Ram,
    pub swap: Swap,
}

impl Memory {
    pub fn new(buf: &mut String) -> Memory {
        
        let memory = read("/proc/meminfo", buf, 0, 0)
            .split('\n').zip(0..).filter(|(_, i)| 
                (0..=2).contains(i) || (14..=15).contains(i)
            ).map(|x| x.0.rsplit(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap()/1024)
            .collect::<Vec<usize>>();

        Memory{
            ram: Ram{
                used: memory[0] - memory[2],
                allocated: memory[0] - memory[1],
                total: memory[0]
            },

            swap: Swap {
                allocated: memory[3] - memory[4],
                total: memory[3]
            }
        }
    }

    pub async fn update(&mut self, buf: &mut String) {

        let memory = read("/proc/meminfo", buf, 0, 0)
            .split('\n').zip(0..).filter(|(_, i)| 
                (1..=2).contains(i) || i == &15
            ).map(|x| x.0.rsplit(" ").collect::<Vec<&str>>()[1].parse::<usize>().unwrap()/1024)
            .collect::<Vec<usize>>();

        self.ram.used = self.ram.total - memory[1];

        self.ram.allocated = self.ram.total - memory[0];

        self.swap.allocated = self.swap.total - memory[2]
    }
}