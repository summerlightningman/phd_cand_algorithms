use crate::problems::travelling_salesman::types::{Matrix, City};
use crate::algorithms::genetic::algorithm::GeneticAlgorithm;


pub struct TSGeneticAlgorithm {
    pub matrix: Matrix,
    pub algo: GeneticAlgorithm<City>
}

impl TSGeneticAlgorithm {
    fn run(self) {
        self.algo.run()
    }
}