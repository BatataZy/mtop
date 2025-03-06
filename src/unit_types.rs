use crate::ITER;

#[derive(Clone, Debug)]
pub struct Magnitude {
    pub index: usize,
    pub values: Vec<u16>,
    pub average: f32,
}
impl Magnitude {
    pub fn new() -> Self {
        Magnitude{
            index: 0,
            values: vec![0; ITER],
            average: 0.,
    }}

    pub fn add(&mut self, value: u16) {

        self.average -= self.values[self.index] as f32 /ITER as f32;
        self.values[self.index] = value;
        self.average += self.values[self.index] as f32 /ITER as f32;

        self.index = (self.index + 1) % ITER;
    }
}



#[derive(Clone, Debug)]
pub struct Delta {
    index: usize,
    pub values: Vec<u32>,
    pub delta: u32
}
impl Delta {
    pub fn new() -> Self {
        Delta{
            index: 0,
            values: vec![0; ITER],
            delta: 0,
    }}

    pub fn add(&mut self, value: u32) {

        self.delta = value - self.values[self.index];
        self.values[self.index] = value;

        self.index = (self.index + 1) % ITER;
    }
}



#[derive(Clone, Debug)]
pub struct Percent {
    index: usize,
    pub values: Vec<u8>,
    pub average: f32,
}
impl Percent {
    pub fn new() -> Self {
        Percent { 
            index: 0 ,
            values: vec![0; ITER],
            average: 0., 
    }}

    pub fn add(&mut self, value: u8) {

        self.average -= self.values[self.index] as f32 / ITER as f32;
        self.values[self.index] = value;
        self.average += self.values[self.index] as f32 / ITER as f32;

        self.index = (self.index + 1) % ITER;
    }
}



#[derive(Clone, Debug, serde::Serialize)]
pub struct Size {
    pub used: usize,
    pub total: usize,
}
impl Size {
    pub fn new(total:usize) -> Self {
        Size { 
            total: total,
            used: 0,
    }}
}