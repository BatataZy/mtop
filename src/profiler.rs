use easy_cast::Cast;
use std::time;

use crate::{unit_types::Magnitude, ITER, PROFILING};

pub struct Profiler {
    times: Magnitude,
    max: f32,
    timer: u16,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            times: Magnitude::new(""),
            max: 0.,
            timer: 0,
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

            if self.timer == ITER / 2 {
                self.timer = 0;
                println!("{:?} us\n", self.times.average);
            }

            self.timer += 1;
        }

        delta_time.cast()
    }
}
