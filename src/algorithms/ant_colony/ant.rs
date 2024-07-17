use rand::thread_rng;
use rand::seq::SliceRandom;
use super::types::City;

#[derive(Debug)]
pub struct Ant {
  pub path: Vec<City>,
}

impl Ant {
  pub fn new(cities_count: &usize) -> Self {
      let mut path: Vec<City> = (0..*cities_count).collect();
      path.shuffle(&mut thread_rng());

      Self { path }
  }

  pub fn current_city(&self) -> City {
      *self.path.last().unwrap()
  }

  pub fn go_to(&mut self, city: City) {
    self.path.push(city);
  }

  pub fn reset_path(&mut self) {
    self.path = vec![self.current_city()];
  }
}