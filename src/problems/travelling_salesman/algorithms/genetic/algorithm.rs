use crate::problems::travelling_salesman::types::{City};
use crate::algorithms::genetic::algorithm::GeneticAlgorithm;
use crate::algorithms::solution::Solution;


pub struct TSGeneticAlgorithm {
    pub algo: GeneticAlgorithm<City>
}

impl TSGeneticAlgorithm {
    pub fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let population = self.algo.run().unwrap();
        let mut solutions: Vec<Solution> = Vec::new();

        for ind in population.into_iter() {
            if ind.fitness.is_some() {
                let distance = ind.fitnesses[0].unwrap();
                let time = if self.algo.fitness_funcs.len() > 1 {
                    Some(ind.fitnesses[1].unwrap() as usize)
                } else {
                    None
                };

                solutions.push(Solution {
                    path: ind.value,
                    distance,
                    time,
                });
            }

            if solutions.len() == self.algo.solutions_count  {
                break
            }
        }

        Ok(solutions)
    }
}