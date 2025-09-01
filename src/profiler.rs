use easy_cast::Cast;
use std::time;

use crate::{unit_types::Magnitude, PROFILING};

pub struct Profiler {
    times: Magnitude,
    max: u16,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            times: Magnitude::new(""),
            max: 0,
        }
    }

    pub fn update<F>(&mut self, f: F) -> u64
    where
        F: FnOnce(),
    {
        let start = time::Instant::now();

        f();

        let end = time::Instant::now();

        let delta_time = (end - start).as_micros();

        if PROFILING {
            self.times.add(delta_time.try_into().unwrap_or_default());

            if self.max < self.times.average {
                self.max = self.times.average;
            }

            println!("{:?}\n{:?}\n", self.times.average, delta_time);
        }

        delta_time.cast()
    }
}
