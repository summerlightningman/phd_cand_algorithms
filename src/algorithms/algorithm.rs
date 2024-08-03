use super::solution::Solution;

pub trait OptimizationAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &str>;
}

pub trait OptimizationAlgorithmBuilder {
    fn iters_count(self, iters_count: u64) -> Self;
    fn actors_count(self, actors_count: usize) -> Self;
    fn solutions_count(self, solutions_count: usize) -> Self;
}