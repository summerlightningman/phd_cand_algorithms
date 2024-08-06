use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use rand::thread_rng;
use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;
use crate::algorithms::bee_colony::{
    algorithm::BeeColonyAlgorithm,
    types::{ResearchFuncRaw, GenerateFuncRaw}
};
use crate::algorithms::constants::{ACTORS_COUNT, ITERS_COUNT, SOLUTIONS_COUNT};
use crate::algorithms::types::{FitnessFuncRaw, Purpose};
use crate::problems::travelling_salesman::helpers::calculate_distance_with_rules;
use crate::problems::travelling_salesman::types::{Matrix, City, RuleFn};
use super::algorithm::TSBeeColonyAlgorithm;

pub struct BeeColonyAlgorithmBuilder {
    pub matrix: Matrix,
    pub rules: Vec<RuleFn>,
    pub actors_count: usize,
    pub iters_count: usize,
    pub solutions_count: usize,
    pub workers_part: f32,
    pub research_func: ResearchFuncRaw<City>,
}

impl OptimizationAlgorithmBuilder for BeeColonyAlgorithmBuilder {
    fn iters_count(mut self, iters_count: usize) -> Self {
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

impl BeeColonyAlgorithmBuilder {
    pub fn new(
        matrix: Matrix,
        research_func: impl Fn(&Vec<City>, &mut ThreadRng) -> Vec<City> + 'static
    ) -> Self {
        Self {
            matrix: matrix,
            rules: vec![],
            actors_count: ACTORS_COUNT,
            iters_count: ITERS_COUNT,
            solutions_count: SOLUTIONS_COUNT,
            workers_part: 0.7,
            research_func: Box::new(research_func),
        }
    }

    pub fn workers_part(mut self, workers_part: f32) -> Self {
        if workers_part >= 1. || workers_part <= 0. {
            panic!("Workers part value is not correct 0 < {} < 1", workers_part);
        }
        self.workers_part = workers_part;
        self
    }

    pub fn build(self) -> TSBeeColonyAlgorithm {
        let cities_count = self.matrix.len();

        let fitness_funcs = vec![
            Box::new(calculate_distance_with_rules(self.matrix, self.rules)) as FitnessFuncRaw<City>,
        ];

        let generate_func: GenerateFuncRaw<City> = Box::new(move || {
            let mut rng = thread_rng();
            let mut value: Vec<usize> = (0..cities_count).collect();
            value.shuffle(&mut rng);
            value
        });

        TSBeeColonyAlgorithm {
            algo: BeeColonyAlgorithm {
                actors_count: self.actors_count,
                iters_count: self.iters_count,
                solutions_count: self.solutions_count,
                workers_part: self.workers_part,
                purpose: Purpose::Min,
                fitness_funcs,
                research_func: self.research_func,
                generate_func: Box::new(generate_func),
            }
        }
    }

}