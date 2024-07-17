use rand::thread_rng;
use rand::seq::SliceRandom;
use super::algorithm::AntColonyAlgorithm;
use super::types::{City, Matrix, PheromoneMatrix};

#[derive(Debug, Clone)]
pub struct Ant {
  path: Vec<City>,
}

impl Ant {
  pub fn new(cities_count: &usize ) -> Self {
    let mut path: Vec<usize> = (0..*cities_count).collect();
    path.shuffle(&mut thread_rng());

    Self { path }
  }

  pub fn current_city(&self) -> City {
    *self.path.last().unwrap()
  }
}