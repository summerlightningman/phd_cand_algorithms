use super::types::{FoodSource, Fitness};

#[derive(Debug, Clone)]
pub struct Bee<T> {
    pub source: FoodSource<T>,
    pub fitness: Fitness
}