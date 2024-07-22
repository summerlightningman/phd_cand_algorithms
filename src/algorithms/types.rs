use crate::problems::travelling_salesman::types::{City};

#[derive(Clone, Copy)]
pub enum Purpose {
    Min,
    Max,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Solution {
    pub path: Vec<City>,
    pub distance: f64,
}
