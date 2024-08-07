use crate::algorithms::solution::Solution;
use crate::problems::travelling_salesman::types::{City};
use crate::algorithms::bee_colony::algorithm::BeeColonyAlgorithm;
use crate::problems::travelling_salesman::helpers::make_solutions;

pub struct TSBeeColonyAlgorithm {
    pub algo: BeeColonyAlgorithm<City>,
}

impl TSBeeColonyAlgorithm {
    pub fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let population = self.algo.run().unwrap();
        let solutions = make_solutions(population, self.algo.solutions_count, &self.algo.fitness_funcs);

        Ok(solutions)
    }
}