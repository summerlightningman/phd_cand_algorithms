use super::types::Solution;

pub trait OptimizationAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &str>;
}

pub trait OptimizationAlgorithmBuilder {
    fn iters_count(mut self, iters_count: u64) -> Self;
    fn actors_count(mut self, actors_count: u64) -> Self;
    fn solutions_count(mut self, solutions_count: u64) -> Self;
}