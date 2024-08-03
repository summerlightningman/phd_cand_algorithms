use crate::algorithms::solution::Solution;
use crate::problems::travelling_salesman::types::{City};
use crate::algorithms::bee_colony::algorithm::BeeColonyAlgorithm;
use crate::problems::travelling_salesman::helpers;

pub struct TSBeeColonyAlgorithm {
    pub algo: BeeColonyAlgorithm<City>,
}

impl TSBeeColonyAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let workers = self.algo.run().unwrap();
        let mut solutions: Vec<Solution> = Vec::new();

        for bee in workers.into_iter() {
            if bee.fitness.is_some() {
                solutions.push(Solution {
                    path: bee.value,
                    distance: helpers::calculate_distance(bee.value),
                });
            }

            if solutions.len() == self.algo.solutions_count {
                break;
            }
        }

        Ok(solutions)
    }
}