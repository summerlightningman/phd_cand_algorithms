use crate::algorithms::algorithm::OptimizationAlgorithm;
use crate::algorithms::types::Solution;
use crate::problems::travelling_salesman::types::{City};
use crate::algorithms::bee_colony::algorithm::BeeColonyAlgorithm;

pub struct TSBeeColonyAlgorithm {
    pub algo: BeeColonyAlgorithm<City>,
}

impl OptimizationAlgorithm for TSBeeColonyAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let workers = self.algo.run().unwrap();
        let mut solutions: Vec<Solution> = Vec::new();

        for bee in workers.into_iter() {
            if let Some(distance) = bee.fitness {
                solutions.push(Solution {
                    path: bee.source,
                    distance,
                });
            }

            if solutions.len() == self.algo.solutions_count {
                break;
            }
        }

        Ok(solutions)
    }
}