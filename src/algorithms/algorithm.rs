use super::types::Solution;

pub trait OptimizationAlgorithm {
    fn run(&self) -> Result<Vec<Solution>, &str>;
}
