use crate::algorithms::algorithm::OptimizationAlgorithm;
use crate::algorithms::types::Solution;

use super::ant::Ant;
use super::types::{City, Matrix, PheromoneMatrix};
use super::utils::calculate_distance;

use random_choice::random_choice;

#[derive(Debug)]
pub struct AntColonyAlgorithm {
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,
    alpha: f64,
    beta: f64,
    q: f64,
    p: f64,
    matrix: Matrix,
    pheromone_matrix: PheromoneMatrix,
    colony: Vec<Ant>,
    solutions: Vec<Solution>,
}

impl OptimizationAlgorithm for AntColonyAlgorithm {
    fn run(&mut self) -> Result<Vec<Solution>, &str> {
        let cities_count = self.cities_count();
        for _ in 1..=self.iters_count {
            let mut iter_pheromone_matrix: PheromoneMatrix =
                vec![vec![1.; cities_count]; cities_count];
            let mut solutions: Vec<Solution> = Vec::new();

            for ant in &mut self.colony {
                let mut distance: f64 = 0.;

                for _ in 0..cities_count {
                    let probabilities = self.get_probabilities_list(&ant)?;
                    let city = self.select_city(probabilities)?;
                    ant.go_to(city);

                    distance = self.get_ant_distance(&ant);
                    if distance > 0. {
                        iter_pheromone_matrix[ant.previous_city()][city] += self.q / distance
                    }
                }

                if ant.path.len() == self.cities_count() {
                    solutions.push(Solution {
                        path: ant.path.clone(),
                        distance,
                    });
                }
                ant.reset_path();
            }

            self.vape_pheromone(&iter_pheromone_matrix);
        }

        Ok(Vec::new())
    }
}

impl AntColonyAlgorithm {
    pub fn new(
        iters_count: usize,
        solutions_count: usize,
        actors_count: usize,
        alpha: f64,
        beta: f64,
        q: f64,
        p: f64,
        matrix: Matrix,
    ) -> Self {
        let cities_count = matrix.len();
        let pheromone_matrix = vec![vec![1.; cities_count]; cities_count];
        let colony = (0..actors_count).map(|_| Ant::new(&cities_count)).collect();

        Self {
            iters_count,
            solutions_count,
            actors_count,
            alpha,
            beta,
            q,
            p,
            colony,
            matrix,
            pheromone_matrix,
            solutions: Vec::new(),
        }
    }

    fn cities_count(&self) -> usize {
        self.matrix.len()
    }

    fn cities_list(&self) -> Vec<City> {
        (0..self.cities_count()).collect()
    }

    fn get_ant_visibility(&self, ant: &Ant, city: &City) -> f64 {
        if ant.path.contains(&city) {
            return 0.;
        }

        let distance = self.matrix[ant.current_city()][*city];
        if distance == 0. {
            return 0.;
        }

        1. / distance
    }

    fn get_pheromone(&self, ant: &Ant, city: &City) -> f64 {
        self.pheromone_matrix[ant.current_city()][*city]
    }

    fn get_ant_preferences(&self, ant: &Ant) -> Vec<f64> {
        let get_ant_preference_to = |city: &City| -> f64 {
            if ant.path.contains(&city) {
                return 0.;
            }

            let visibility = self.get_ant_visibility(&ant, &city);
            let pheromone = self.get_pheromone(&ant, &city);

            visibility.powf(self.alpha) * pheromone.powf(self.beta)
        };

        self.cities_list()
            .iter()
            .map(get_ant_preference_to)
            .collect()
    }

    fn get_probabilities_list(&self, ant: &Ant) -> Result<Vec<f64>, &str> {
        let cities_preferences = self.get_ant_preferences(ant);
        let cities_preferences_sum: f64 = cities_preferences.iter().sum();

        if cities_preferences_sum == 0. {
            Err("Error calculating")
        } else {
            Ok(self
                .cities_list()
                .iter()
                .map(|city: &City| {
                    let city_preference = cities_preferences[*city];
                    city_preference / cities_preferences_sum
                })
                .collect())
        }
    }

    fn select_city(&self, probabilities: Vec<f64>) -> Result<City, &str> {
        let cities_list: Vec<City> = self.cities_list();
        let selected_cities = random_choice().random_choice_f64(&cities_list, &probabilities, 1);
        if selected_cities.len() == 0 {
            Err("Ant haven't found a path")
        } else {
            Ok(*selected_cities[0])
        }
    }

    fn vape_pheromone(&mut self, iter_pheromone_matrix: &PheromoneMatrix) {
        let cities_count = self.cities_count();
        for i in 0..cities_count {
            for j in 0..cities_count {
                self.pheromone_matrix[i][j] =
                    self.pheromone_matrix[i][j] * (1. - self.p) + iter_pheromone_matrix[i][j]
            }
        }
    }

    fn get_ant_distance(&self, ant: &Ant) -> f64 {
        if ant.path.len() <= 1 {
            0.
        } else {
            calculate_distance(&ant.path, &self.matrix)
        }
    }
}
