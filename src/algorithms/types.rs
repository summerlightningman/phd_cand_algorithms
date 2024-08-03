use crate::algorithms::individual::Individual;
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

pub type Fitness = Option<f32>;
pub type FitnessFuncRaw<T> = Box<dyn Fn(&Vec<T>) -> Fitness>;
pub type FitnessFuncs<T> = Vec<FitnessFuncRaw<T>>;
pub type Population<T> = Vec<Individual<T>>;