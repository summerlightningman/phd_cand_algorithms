use std::cell::RefCell;
use rand::thread_rng;
use crate::algorithms::ant_colony::algorithm::AntColonyAlgorithm as Parent;
use crate::algorithms::ant_colony::ant::Ant;
use crate::algorithms::ant_colony::types::{City, PheromoneMatrix};
use crate::algorithms::ant_colony::utils::calculate_distance;
use crate::algorithms::types::Solution;
use crate::problems::travelling_salesman::rules::Rule;
use crate::problems::travelling_salesman::rules::apply_rules;
use lru::LruCache;

struct TSAntColonyAlgorithm {
    algo: Parent,
    rules: Vec<Rule>,
    penalty_cache: RefCell<LruCache<Vec<City>, Option<f64>>>
}

impl TSAntColonyAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &str> {
        let cities_count = self.algo.cities_count();
        let mut pheromone_matrix: PheromoneMatrix = Parent::generate_pheromone_matrix(cities_count);
        let mut solutions: Vec<Solution> = Vec::new();
        let mut rng = thread_rng();
        let mut colony: Vec<Ant> = (0..self.algo.actors_count).map(|_| Ant::new(cities_count, &mut rng)).collect();

        'outer: for _ in 1..=self.algo.iters_count {
            let mut iter_pheromone_matrix: PheromoneMatrix = Parent::generate_pheromone_matrix(cities_count);

            for ant in colony.iter_mut() {
                let mut distance: f64 = 0.;

                for _ in 0..cities_count - 1 {
                    let probabilities = self.algo.get_probabilities_list(&ant, &mut pheromone_matrix)?;
                    let city = self.algo.select_city(probabilities)?;
                    ant.go_to(city);

                    match self.get_ant_distance(&ant) {
                        Some(d) if d > 0. => {
                            distance = d;
                            iter_pheromone_matrix[ant.previous_city()][city] += self.algo.q / d
                        },
                        None => continue 'outer,
                        _ => continue
                    }
                }

                if ant.path.len() == self.algo.cities_count() {
                    solutions.push(Solution {
                        path: ant.path.clone(),
                        distance,
                    });
                }
                ant.reset_path();
            }

            solutions.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
            solutions = solutions[..solutions.len().min(self.algo.solutions_count)].to_owned();

            self.algo.vape_pheromone(&mut pheromone_matrix, &iter_pheromone_matrix);
        }

        Ok(solutions.clone())
    }

    fn get_ant_distance(&self, ant: &Ant) -> Option<f64> {
        if ant.path.len() <= 1 {
            Some(0.)
        } else {
            match self.get_penalty(&ant.path) {
                Some(penalty) => Some(calculate_distance(&ant.path, &self.algo.matrix) + penalty),
                None => None
            }
        }
    }

    fn get_ant_visibility(&self, ant: &Ant, city: &City) -> f64 {
        if ant.path.contains(&city) {
            return 0.;
        }

        let path = vec![ant.current_city(), *city];
        let penalty = if let Some(p) = self.get_penalty(&path) {
            p
        } else {
            return 0.
        };

        let distance = self.algo.matrix[ant.current_city()][*city] + penalty;
        if distance == 0. {
            return 0.;
        }

        1. / distance
    }

    fn get_penalty(&self, path: &Vec<City>) -> Option<f64> {
        let mut cache = self.penalty_cache.borrow_mut();
        if let Some(&cached_result) = cache.get(path) {
            return cached_result;
        }

        let result = match apply_rules(path, &self.rules) {
            Some(penalty) => Some(penalty as f64),
            None => None
        };

        cache.put(path.clone(), result);
        result
    }
}