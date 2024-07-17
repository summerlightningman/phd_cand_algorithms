use super::types::{ CollectFunc, Purpose, CalculationResult };
use rand::prelude::StdRng;

pub trait OptimizationAlgorithm<T: Clone> {
    fn new(
        iters_count: usize,
        solutions_count: usize,
        actors_count: usize,
        purpose: Purpose,
        top_results: Vec<Result<T>>,
        std: StdRng,
    ) -> Self;

    fn run(&mut self) -> Vec<Result<T>>;
    fn update_top_results(&mut self, new_results: &[CalculationResult<T>]);
    fn summarize_results(&mut self, new_results: &[CalculationResult<T>]);
    fn by_fitness() -> f64 {
        result.1
    }
}
