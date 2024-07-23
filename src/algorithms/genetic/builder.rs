use super::types::{FitnessFunc, CrossoverFunc, MutateFunc, SelectFunc};
use crate::algorithms::constants::{ACTORS_COUNT, SOLUTIONS_COUNT, ITERS_COUNT};
use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;

struct GeneticAlgorithmBuilder<T> {
    fitness_func: FitnessFunc<T>,
    actors_count: u64,
    iters_count: u64,
    solutions_count: u64,
    p_mutation: f32,
    crossover_func: CrossoverFunc<T>,
    mutate_func: MutateFunc<T>,
    select_func: SelectFunc<T>,
}

impl<T> OptimizationAlgorithmBuilder for GeneticAlgorithmBuilder<T> {
    fn iters_count(mut self, iters_count: u64) -> Self {
        self.iters_count = iters_count;
        self
    }

    fn actors_count(mut self, actors_count: u64) -> Self {
        self.actors_count = actors_count;
        self
    }

    fn solutions_count(mut self, solutions_count: u64) -> Self {
        self.solutions_count = solutions_count;
        self
    }
}

impl<T> GeneticAlgorithmBuilder<T> {
    fn new(
        fitness_func: FitnessFunc<T>,
        crossover_func: CrossoverFunc<T>,
        mutate_func: MutateFunc<T>,
        select_func: SelectFunc<T>
    ) -> Self {
        Self {
            solutions_count: SOLUTIONS_COUNT,
            actors_count: ACTORS_COUNT,
            iters_count: ITERS_COUNT,
            p_mutation: 0.3,
            fitness_func,
            crossover_func,
            mutate_func,
            select_func,
        }
    }

    fn p_mutation(mut self, p_mutation: f32) -> Self {
        if p_mutation < 1. {
            self.p_mutation = p_mutation;
            self
        } else {
            panic!("Value must be 0 <= p_mutation < 1")
        }
    }
}