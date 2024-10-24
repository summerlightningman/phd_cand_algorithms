use crate::algorithms::genetic::algorithm::GeneticAlgorithm;
use crate::problems::travelling_salesman::helpers::make_solutions;
use crate::problems::travelling_salesman::solution::Solution;
use crate::problems::travelling_salesman::types::City;

pub struct TSGeneticAlgorithm {
    pub algo: GeneticAlgorithm<City>,
}

impl TSGeneticAlgorithm {
    pub fn run(&self) -> Result<Vec<Solution>, &'static str> {
        let population = self.algo.run().unwrap();
        let solutions: Vec<Solution> = make_solutions(
            population,
            self.algo.solutions_count,
            &self.algo.fitness_funcs,
        );

        Ok(solutions)
    }
}
