pub trait OptimizationAlgorithmBuilder {
    fn iters_count(self, iters_count: usize) -> Self;
    fn actors_count(self, actors_count: usize) -> Self;
    fn solutions_count(self, solutions_count: usize) -> Self;
}