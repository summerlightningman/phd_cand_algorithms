#[derive(Clone, Copy)]
pub enum Purpose {
    Min,
    Max,
}

pub type City = usize;

pub struct Solution {
    pub path: Vec<City>,
    pub distance: f32,
}
