use crate::algorithms::algorithm::OptimizationAlgorithm;
use crate::algorithms::types::Solution;

use super::ant::Ant;
use super::types::{City, Matrix, PheromoneMatrix};
use super::utils::calculate_distance;

use random_choice::random_choice;

#[derive(Debug)]
pub struct AntColonyAlgorithm {
    iters_count: usize,
    solutions_count: usize,
    alpha: f64,
    beta: f64,
    q: f64,
    p: f64,
    matrix: Matrix,
    colony: Vec<Ant>,
    solutions: Vec<Solution>,
}

impl OptimizationAlgorithm for AntColonyAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &str> {
        let cities_count = self.cities_count();
        let mut pheromone_matrix: PheromoneMatrix = Self::generate_pheromone_matrix(cities_count);
        let mut solutions: Vec<Solution> = Vec::new();

        for _ in 1..=self.iters_count {
            let mut iter_pheromone_matrix: PheromoneMatrix = Self::generate_pheromone_matrix(cities_count);
            let mut colony = self.colony.clone();

            for ant in colony.iter_mut() {
                let mut distance: f64 = 0.;

                for _ in 0..cities_count {
                    let probabilities = self.get_probabilities_list(&ant, &mut pheromone_matrix)?;
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

            solutions.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap());
            solutions = solutions[..solutions.len().min(self.solutions_count)].to_owned();

            self.vape_pheromone(&mut pheromone_matrix, &iter_pheromone_matrix);
        }

        Ok(self.solutions.clone())
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
        let colony = (0..actors_count).map(|_| Ant::new(cities_count)).collect();

        Self {
            iters_count,
            solutions_count,
            alpha,
            beta,
            q,
            p,
            colony,
            matrix,
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

    fn generate_pheromone_matrix(cities_count: usize) -> PheromoneMatrix {
        vec![vec![1.; cities_count]; cities_count]
    }

    fn get_ant_preferences(&self, ant: &Ant, pheromone_matrix: &mut PheromoneMatrix) -> Vec<f64> {
        let get_ant_preference_to = |city: &City| -> f64 {
            if ant.path.contains(&city) {
                return 0.;
            }

            let visibility = self.get_ant_visibility(&ant, &city);
            let pheromone = pheromone_matrix[ant.current_city()][*city];

            visibility.powf(self.alpha) * pheromone.powf(self.beta)
        };

        self.cities_list()
            .iter()
            .map(get_ant_preference_to)
            .collect()
    }

    fn get_probabilities_list(&self, ant: &Ant, pheromone_matrix: &mut PheromoneMatrix) -> Result<Vec<f64>, &str> {
        let cities_preferences = self.get_ant_preferences(ant, pheromone_matrix);
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

    fn vape_pheromone(&self, pheromone_matrix: &mut PheromoneMatrix, iter_pheromone_matrix: &PheromoneMatrix) {
        let cities_count = self.cities_count();

        for i in 0..cities_count {
            for j in 0..cities_count {
                pheromone_matrix[i][j] =
                    pheromone_matrix[i][j] * (1. - self.p) + iter_pheromone_matrix[i][j]
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
