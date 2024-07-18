#[derive(Clone, Copy)]
pub enum Purpose {
    Min,
    Max,
}

pub type City = usize;

#[derive(Debug)]
pub struct Solution {
    pub path: Vec<City>,
    pub distance: f64,
}
