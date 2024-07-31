use rand::rngs::ThreadRng;
use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;
use crate::algorithms::bee_colony::types::{FitnessFunc, GenerateFunc};
use crate::algorithms::types::Purpose;
use crate::algorithms::constants::{ACTORS_COUNT, SOLUTIONS_COUNT, ITERS_COUNT};
use super::algorithm::BeeColonyAlgorithm;

pub struct BeeColonyAlgorithmBuilder<T> {
    pub actors_count: usize,
    pub iters_count: u64,
    pub solutions_count: usize,
    pub workers_part: f32,
    pub purpose: Purpose,
    pub fitness_func: FitnessFunc<T>,
    pub research_func: Box<dyn Fn(&Vec<T>, &mut ThreadRng) -> Vec<T>>,
    pub generate_func: GenerateFunc<T>,
}

impl<T> OptimizationAlgorithmBuilder for BeeColonyAlgorithmBuilder<T> {
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

impl<T: 'static> BeeColonyAlgorithmBuilder<T> {
    pub fn new(
        fitness_func: FitnessFunc<T>,
        generate_func: GenerateFunc<T>,
        research_func: impl Fn(&Vec<T>, &mut ThreadRng) -> Vec<T> + 'static
    ) -> Self {
        Self {
            actors_count: ACTORS_COUNT,
            iters_count: ITERS_COUNT,
            solutions_count: SOLUTIONS_COUNT,
            workers_part: 0.7,
            purpose: Purpose::Min,
            research_func: Box::new(research_func),
            fitness_func,
            generate_func,
        }
    }

    pub fn workers_part(mut self, workers_part: f32) -> Self {
        if workers_part >= 1. || workers_part <= 0. {
            panic!("Workers part value is not correct 0 < {} < 1", workers_part);
        }
        self.workers_part = workers_part;
        self
    }

    pub fn purpose(mut self, purpose: Purpose) -> Self {
        self.purpose = purpose;
        self
    }

    pub fn build(self) -> BeeColonyAlgorithm<T> {
        BeeColonyAlgorithm {
            actors_count: self.actors_count,
            iters_count: self.iters_count,
            solutions_count: self.solutions_count,
            workers_part: self.workers_part,
            purpose: self.purpose,
            fitness_func: Box::new(self.fitness_func),
            research_func: self.research_func,
            generate_func: Box::new(self.generate_func),
        }
    }
}