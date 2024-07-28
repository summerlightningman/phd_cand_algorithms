use super::types::{FitnessFunc, CrossoverFunc, GenerateFunc};
use super::algorithm::GeneticAlgorithm;
use crate::algorithms::constants::{ACTORS_COUNT, SOLUTIONS_COUNT, ITERS_COUNT};
use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;
use crate::algorithms::types::Purpose;
use crate::algorithms::genetic::types::Population;

pub struct GeneticAlgorithmBuilder<T> {
    fitness_func: FitnessFunc<T>,
    actors_count: usize,
    iters_count: u64,
    solutions_count: usize,
    p_mutation: f32,
    crossover_func: CrossoverFunc<T>,
    mutate_func: Box<dyn Fn(Vec<T>) -> Vec<T>>,
    select_func: Box<dyn Fn(Population<T>, &Purpose) -> Population<T>>,
    generate_func: GenerateFunc<T>,
    purpose: Purpose,
}

impl<T> OptimizationAlgorithmBuilder for GeneticAlgorithmBuilder<T> {
    fn iters_count(mut self, iters_count: u64) -> Self {
        self.iters_count = iters_count;
        self
    }

    fn actors_count(mut self, actors_count: usize) -> Self {
        self.actors_count = actors_count;
        self
    }

    fn solutions_count(mut self, solutions_count: usize) -> Self {
        self.solutions_count = solutions_count;
        self
    }
}

impl<T: 'static> GeneticAlgorithmBuilder<T> {
    pub fn new(
        fitness_func: FitnessFunc<T>,
        crossover_func: CrossoverFunc<T>,
        mutate_func: impl Fn(Vec<T>) -> Vec<T> + 'static,
        select_func: impl Fn(Population<T>, &Purpose) -> Population<T> + 'static,
        generate_func: GenerateFunc<T>,
    ) -> Self {
        Self {
            solutions_count: SOLUTIONS_COUNT,
            actors_count: ACTORS_COUNT,
            iters_count: ITERS_COUNT,
            p_mutation: 0.3,
            purpose: Purpose::Min,
            fitness_func,
            crossover_func,
            mutate_func: Box::new(mutate_func),
            select_func: Box::new(select_func),
            generate_func
        }
    }

    pub fn p_mutation(mut self, p_mutation: f32) -> Self {
        if p_mutation < 1. {
            self.p_mutation = p_mutation;
            self
        } else {
            panic!("Value must be 0 <= p_mutation < 1")
        }
    }

    pub fn purpose(mut self, purpose: Purpose) -> Self {
        self.purpose = purpose;
        self
    }

    pub fn build(self) -> GeneticAlgorithm<T> {
        GeneticAlgorithm {
            fitness_func: Box::new(self.fitness_func),
            generate_func: Box::new(self.generate_func),
            actors_count: self.actors_count,
            iters_count: self.iters_count,
            solutions_count: self.solutions_count,
            p_mutation: self.p_mutation,
            crossover_func: self.crossover_func,
            mutate_func: self.mutate_func,
            select_func: self.select_func,
            purpose: self.purpose,
        }
    }
}