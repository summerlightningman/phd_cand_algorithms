use super::types::Solution;

pub trait OptimizationAlgorithm {
    fn run(&mut self) -> Vec<Solution>;
    // fn update_top_results(&mut self, new_results: &Solution);
    // fn summarize_results(&mut self, new_results: &Solution);
}
