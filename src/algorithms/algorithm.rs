use super::types::Solution;

pub trait OptimizationAlgorithm {
    fn run(&mut self) -> Result<Vec<Solution>, &str>;
    // fn update_top_results(&mut self, new_results: &Solution);
    // fn summarize_results(&mut self, new_results: &Solution);
}
