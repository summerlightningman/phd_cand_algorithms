use super::types::City;
use rand::seq::SliceRandom;
use rand::thread_rng;

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

    pub fn previous_city(&self) -> City {
        *self.path.get(self.path.len() - 2).unwrap()
    }

    pub fn go_to(&mut self, city: City) {
        self.path.push(city);
    }

    pub fn reset_path(&mut self) {
        self.path = vec![self.current_city()];
    }
}
