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
    index: usize,
    values: Vec<u32>,
    pub average: u32,
}
impl Delta {
    pub fn new() -> Self {
        Self {
            index: 0,
            values: vec![0; ITER.into()],
            average: 0,
        }
    }

    pub fn add(&mut self, value: u32) {
        self.average = value - self.values[self.index];
        self.values[self.index] = value;

        self.index = (self.index + 1) % ITER as usize;
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
