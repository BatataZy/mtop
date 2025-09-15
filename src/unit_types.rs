use std::collections::VecDeque;

use crate::{io::read, ITER};

#[derive(Debug)]
pub struct Magnitude {
    file: String,
    values: VecDeque<u16>,
    pub average: u16,
}
impl Magnitude {
    pub fn new(file: &str) -> Self {
        Self {
            file: file.to_owned(),
            values: VecDeque::with_capacity(ITER.into()),
            average: 0,
        }
    }

    pub fn add(&mut self, value: u16) {
        if self.values.len() == ITER as usize {
            self.average -= self.values.pop_back().unwrap_or(0) / ITER;
        }

        self.average += value / ITER;

        self.values.push_front(value);
    }

    pub fn read(&self, start: usize, end: usize) -> String {
        read(&self.file, start, end)
    }
}

#[derive(Clone, Debug)]
pub struct Delta {
    values: VecDeque<u32>,
    pub average: u32,
}
impl Delta {
    pub fn new() -> Self {
        Self {
            values: VecDeque::with_capacity(ITER.into()),
            average: 0,
        }
    }

    pub fn add(&mut self, value: u32) {
        self.values.push_front(value);

        self.average = value - self.values.back().expect("vecdeque is not empty");

        if self.values.len() == ITER as usize {
            self.values.pop_back();
        }
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
