use std::time;
//use std::collections::BinaryHeap;

use crate::{unit_types::Magnitude, PROFILING};

pub struct Profiler {
        times: Magnitude,
        max: u16
    }

impl Profiler {
    pub fn new() -> Profiler {
        Profiler{
            times: Magnitude::new(),
            max: 0
        }
    }

    pub async fn update<F>(&mut self, mut f: F) -> u64 where
        F: FnMut() -> () {

        let start = time::Instant::now();

        f();

        let end = time::Instant::now();

        let delta_time = (end-start).as_micros() as u16;

        if PROFILING {

            self.times.add(delta_time);

            if self.max < self.times.average as u16 {self.max = self.times.average as u16}

            if self.times.index == 0 {
                println!("{:?}", (self.times.average, self.max));
                self.max = 0
            }
        }

        return delta_time as u64;

    }
}