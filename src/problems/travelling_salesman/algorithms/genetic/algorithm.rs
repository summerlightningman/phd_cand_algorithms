use crate::problems::travelling_salesman::types::{City};
use crate::algorithms::genetic::algorithm::GeneticAlgorithm;
use crate::algorithms::solution::Solution;


pub struct TSGeneticAlgorithm {
    pub algo: GeneticAlgorithm<City>
}

impl TSGeneticAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let population = self.algo.run().unwrap();
        let mut solutions: Vec<Solution> = Vec::new();

        for ind in population.into_iter() {
            if let Some(_) = ind.fitness {
                solutions.push(Solution {
                    path: ind.value,
                    distance: 0.,
                    time: None,
                });
            }

            if solutions.len() == self.algo.solutions_count  {
                break
            }
        }

        Ok(solutions)
    }
}