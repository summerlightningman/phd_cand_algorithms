use crate::algorithms::genetic::types::FitnessFunc;

struct GeneticAlgorithmBuilder<T> {
    fitness_func: FitnessFunc<T>,
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,

}