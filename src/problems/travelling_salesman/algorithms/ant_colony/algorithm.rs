use lru::LruCache;
use std::cell::RefCell;
use rand::thread_rng;
use crate::algorithms::ant_colony::algorithm::AntColonyAlgorithm as Parent;
use crate::algorithms::ant_colony::ant::Ant;
use crate::algorithms::ant_colony::types::{City, PheromoneMatrix};
use crate::algorithms::solution::Solution;
use crate::problems::travelling_salesman::helpers;
use crate::problems::travelling_salesman::types::{RuleFn, TimeMatrix};

pub struct TSAntColonyAlgorithm {
    pub algo: Parent,
    pub rules: Vec<RuleFn>,
    pub time_matrix: Option<TimeMatrix>,
    pub penalty_cache: RefCell<LruCache<Vec<City>, Option<f64>>>,
}

impl TSAntColonyAlgorithm {
    pub fn run(&self) -> Result<Vec<Solution>, &str> {
        let cities_count = self.algo.cities_count();
        let mut pheromone_matrix: PheromoneMatrix = Parent::generate_pheromone_matrix(cities_count);
        let mut solutions: Vec<Solution> = Vec::new();
        let mut rng = thread_rng();
        let mut colony: Vec<Ant> = (0..self.algo.actors_count).map(|_| Ant::new(cities_count, &mut rng)).collect();

        'outer: for _ in 1..=self.algo.iters_count {
            let mut iter_pheromone_matrix: PheromoneMatrix = Parent::generate_pheromone_matrix(cities_count);

            for ant in colony.iter_mut() {
                for _ in 0..cities_count - 1 {
                    let probabilities = self.algo.get_probabilities_list(&ant, &mut pheromone_matrix)?;
                    let city = self.algo.select_city(probabilities)?;
                    ant.go_to(city);

                    match self.get_ant_distance(&ant) {
                        Some(d) if d > 0. => {
                            ant.distance = d;
                            if let Some(time_matrix) = &self.time_matrix {
                                ant.time = Some(helpers::calculate_time(time_matrix, &ant.path))
                            }
                            iter_pheromone_matrix[ant.previous_city()][city] += self.algo.q / d
                        },
                        None => continue 'outer,
                        _ => continue
                    }
                }

                if ant.path.len() == self.algo.cities_count() {
                    solutions.push(Solution {
                        path: ant.path.clone(),
                        distance: ant.distance,
                        time: ant.time
                    });
                }
                ant.reset_path();
            }

            let distance_min = colony.iter().min_by(|a, b| {
                a.distance.partial_cmp(&b.distance).unwrap()
            }).unwrap().distance;
            let distance_max = colony.iter().max_by(|a, b| {
                a.distance.partial_cmp(&b.distance).unwrap()
            }).unwrap().distance;
            let distance_diff = distance_max - distance_min;

            let time_min = if self.time_matrix.is_none() {
                1.
            } else {
                colony.iter().min_by(|a, b| {
                    let a_time = a.time.unwrap();
                    let b_time = b.time.unwrap();

                    a_time.partial_cmp(&b_time).unwrap()
                }).unwrap().time.unwrap() as f64
            };
            let time_max = if self.time_matrix.is_none() {
                1.
            } else {
                colony.iter().max_by(|a, b| {
                    let a_time = a.time.unwrap();
                    let b_time = b.time.unwrap();

                    a_time.partial_cmp(&b_time).unwrap()
                }).unwrap().time.unwrap() as f64
            };
            let time_diff = time_max - time_min;

            solutions.sort_by(|a, b| {
                let a_distance_norm = (a.distance - distance_min) / distance_diff;
                let b_distance_norm = (b.distance - distance_min) / distance_diff;

                if self.time_matrix.is_some() {
                    let a_time_norm = (a.time.unwrap() as f64 - time_min) / time_diff;
                    let b_time_norm = (a.time.unwrap() as f64 - time_min) / time_diff;
                    (a_distance_norm + a_time_norm).partial_cmp(&(b_distance_norm + b_time_norm)).unwrap()
                } else {
                    a_distance_norm.partial_cmp(&b_distance_norm).unwrap()
                }
            });

            solutions.truncate(self.algo.solutions_count);
            self.algo.vape_pheromone(&mut pheromone_matrix, &iter_pheromone_matrix);
        }

        Ok(solutions)
    }

    fn get_ant_distance(&self, ant: &Ant) -> Option<f64> {
        if ant.path.len() <= 1 {
            Some(0.)
        } else {
            let mut cache = self.penalty_cache.borrow_mut();
            if let Some(result) = cache.get(&ant.path.clone()) {
                *result
            } else {
                let result = self.calculate_distance(ant);
                cache.put(ant.path.clone(), result);
                result
            }
        }
    }

    fn get_ant_visibility(&self, ant: &Ant, city: &City) -> f64 {
        if ant.path.contains(&city) {
            return 0.;
        }

        let penalty = self.get_penalty_to_city(&ant.path, city);
        if let None = penalty {
            return 0.;
        }

        let distance = self.algo.matrix[ant.current_city()][*city] + penalty.unwrap();
        if distance == 0. {
            return 0.;
        }

        let time = match &self.time_matrix {
            Some(time_matrix) => time_matrix[ant.current_city()][*city],
            None => 0
        } as f64;

        1. / (distance + time)
    }

    fn calculate_distance(&self, ant: &Ant) -> Option<f64> {
        let penalty = if self.rules.is_empty() {
            0.
        } else {
            let mut p = 0;
            for evaluate in self.rules.iter() {
                match evaluate(&ant.path) {
                    Some(pen) => p += pen,
                    None => return None
                }
            }
            p as f64
        };

        Some(helpers::calculate_distance(&self.algo.matrix, &ant.path) + penalty)
    }

    fn get_ant_time(&self, path: &Vec<City>) -> usize {
        if let Some(time_matrix) = &self.time_matrix {
            helpers::calculate_time(time_matrix, path)
        } else {
            1
        }
    }

    fn get_penalty_to_city(&self, path: &Vec<City>, city: &City) -> Option<f64> {
        let mut cache = self.penalty_cache.borrow_mut();
        let mut path = path.clone();
        path.push(*city);
        if let Some(result) = cache.get(&path) {
            *result
        } else {
            let mut sum = 0;
            for evaluate in self.rules.iter() {
                if let Some(penalty) = evaluate(&path) {
                    sum += penalty;
                } else {
                    return None
                }
            }

            cache.put(path.clone(), Some(sum as f64));
            Some(sum as f64)
        }
    }
}