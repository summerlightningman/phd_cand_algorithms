use crate::algorithms::solution::Solution;
use crate::problems::travelling_salesman::types::{City};
use crate::algorithms::bee_colony::algorithm::BeeColonyAlgorithm;

pub struct TSBeeColonyAlgorithm {
    pub algo: BeeColonyAlgorithm<City>,
}

impl TSBeeColonyAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let workers = self.algo.run().unwrap();
        let mut solutions: Vec<Solution> = Vec::new();

        for bee in workers.into_iter() {
            if bee.fitness.is_some() {
                let distance = bee.fitnesses[0].unwrap();
                let time = if self.algo.fitness_funcs.len() > 1 {
                    Some(bee.fitnesses[1].unwrap() as usize)
                } else {
                    None
                };

                solutions.push(Solution {
                    path: bee.value,
                    distance,
                    time
                });
            }

            if solutions.len() == self.algo.solutions_count {
                break;
            }
        }

        Ok(solutions)
    }
}