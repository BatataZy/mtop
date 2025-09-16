use std::collections::VecDeque;

use crate::{io::read, ITER};

#[derive(Debug)]
pub struct Magnitude {
    file: String,
    values: VecDeque<u16>,
    pub average: f32,
}
impl Magnitude {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_owned(),
            values: VecDeque::from(vec![0; ITER.into()]),
            average: 0.,
        }
    }

    pub fn add(&mut self, value: u16) {
        self.average -= f32::from(
            self.values
                .pop_front()
                .expect("vecdeque shouldn't be empty"),
        ) / f32::from(ITER);

        self.average += f32::from(value) / f32::from(ITER);

        self.values.push_back(value);
    }

    pub fn read(&self, start: usize, end: usize) -> String {
        read(&self.file, start, end)
    }
}

#[derive(Clone, Debug)]
pub struct Delta {
    values: VecDeque<usize>,
    pub average: usize,
}
impl Delta {
    pub fn new() -> Self {
        Self {
            values: VecDeque::from(vec![0; ITER.into()]),
            average: 0,
        }
    }

    pub fn add(&mut self, value: usize) {
        self.average = value.saturating_sub(
            self.values
                .pop_front()
                .expect("vecdeque shouldn't be empty"),
        );

        self.values.push_back(value);
    }
}

#[derive(Clone, Debug, serde::Serialize)]
pub struct Size {
    file: String,
    pub used: usize,
    pub total: usize,
}
impl Size {
    pub fn new(file: &str, total: usize) -> Self {
        Self {
            file: file.to_owned(),
            total,
            used: 0,
        }
    }
    pub fn read(&self, start: usize, end: usize) -> String {
        read(&self.file, start, end)
    }
}
