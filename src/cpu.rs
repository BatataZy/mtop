use crate::{
    io::read,
    unit_types::{Delta, Magnitude},
};

#[derive(Debug)]
pub struct Cpu {
    pub threads: u16,
    pub clock: Vec<Magnitude>,
    pub util: Vec<u16>,
    time: (Vec<Delta>, Vec<Delta>),
    pub temp: f32,
}

impl Cpu {
    pub fn new() -> Self {
        let threads: u16 = read("/sys/devices/system/cpu/online", 2, 0)
            .parse::<u16>()
            .expect("fully numeric string, hopefully smaller than 16 bits")
            + 1;

        let clock = (0..threads)
            .map(|t| {
                Magnitude::new(
                    &("/sys/devices/system/cpu/cpu".to_owned()
                        + &t.to_string()
                        + "/cpufreq/scaling_cur_freq"),
                )
            })
            .collect();

        Self {
            threads,
            clock,
            util: vec![0; threads as usize],
            time: (
                vec![Delta::new(); threads as usize],
                vec![Delta::new(); threads as usize],
            ),
            temp: 0.,
        }
    }

    pub fn update(&mut self) {
        //CPU CLOCK LOGIC
        self.clock.iter_mut().for_each(|clock| {
            clock.add(
                clock
                    .read(0, 3)
                    .parse::<u16>()
                    .expect("fully numeric string"),
            );
        });

        //CPU UTIL LOGIC
        let times: Vec<String> = read("/proc/stat", 0, 0)
            .split('\n')
            .skip(1)
            .filter_map(|line| line.strip_prefix("cpu"))
            .map(str::to_string)
            .collect();

        self.time
            .0
            .iter_mut()
            .zip(self.time.1.iter_mut())
            .zip(self.util.iter_mut())
            .zip(times)
            .map(|(((total, idle), util), all)| {
                (
                    total,
                    idle,
                    util,
                    all.split(' ')
                        .skip(1)
                        .map(|s| s.parse::<u32>().expect("fully numeric string"))
                        .collect::<Vec<u32>>(),
                )
            })
            .for_each(|(total, idle, util, all)| {
                total.add(all.iter().sum());
                idle.add(all[3] + all[4]);

                *util = 100
                    - u16::try_from(idle.average * 100 / total.average)
                        .expect("number is always smaller than 100");
            });

        self.temp = 0.0;
    }
}
