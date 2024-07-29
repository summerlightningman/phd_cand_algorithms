use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;
use crate::algorithms::constants::{ACTORS_COUNT, ITERS_COUNT, SOLUTIONS_COUNT};
use crate::algorithms::genetic::algorithm::GeneticAlgorithm;
use crate::algorithms::genetic::types::{FitnessFuncRaw, GenerateFuncRaw, Population};
use crate::problems::travelling_salesman::algorithms::genetic::algorithm::TSGeneticAlgorithm;
use crate::problems::travelling_salesman::helpers;
use crate::problems::travelling_salesman::types::{City, Matrix};
use rand::{thread_rng};
use rand::prelude::SliceRandom;
use crate::algorithms::genetic::methods::Crossover;
use crate::algorithms::types::Purpose;
use crate::problems::travelling_salesman::rules::{apply_rules, parse_rules, Rule};

pub struct TSGeneticAlgorithmBuilder {
    matrix: Matrix,
    actors_count: usize,
    iters_count: u64,
    solutions_count: usize,
    p_mutation: f32,
    mutate_func: Box<dyn Fn(Vec<City>) -> Vec<City>>,
    select_func: Box<dyn Fn(Population<City>, &Purpose) -> Population<City>>,
    rules: Vec<Rule>,
}

impl OptimizationAlgorithmBuilder for TSGeneticAlgorithmBuilder {
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

impl TSGeneticAlgorithmBuilder {
    pub fn new(
        matrix: Matrix,
        mutate_func: impl Fn(Vec<City>) -> Vec<City> + 'static,
        select_func: impl Fn(Population<City>, &Purpose) -> Population<City> + 'static,
    ) -> Self {
        Self {
            matrix,
            mutate_func: Box::new(mutate_func),
            select_func: Box::new(select_func),
            actors_count: ACTORS_COUNT,
            solutions_count: SOLUTIONS_COUNT,
            iters_count: ITERS_COUNT,
            rules: Vec::new(),
            p_mutation: 0.3,
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

    pub fn rules(mut self, rules: Vec<&'static str>) -> Self {
        self.rules = parse_rules(rules);
        self
    }

    pub fn build(self) -> TSGeneticAlgorithm {
        let cities_count = self.matrix.len();
        let matrix = self.matrix.clone();

        let fitness_func: FitnessFuncRaw<City> = Box::new(move |cities| -> Option<f64> {
            let penalty: i32 = if self.rules.is_empty() {
                0
            } else {
                match apply_rules(cities, &self.rules) {
                    None => return None,
                    Some(p) => p
                }
            };

            Some(helpers::calculate_distance(&self.matrix, &cities) + penalty as f64)
        });

        let generate_func: GenerateFuncRaw<City> = Box::new(move || {
            let mut rng = thread_rng();
            let mut value: Vec<usize> = (0..cities_count).collect();
            value.shuffle(&mut rng);
            value
        });

        TSGeneticAlgorithm {
            matrix,
            algo: GeneticAlgorithm {
                fitness_func,
                generate_func,
                purpose: Purpose::Min,
                actors_count: self.actors_count,
                iters_count: self.iters_count,
                solutions_count: self.solutions_count,
                p_mutation: self.p_mutation,
                crossover_func: Crossover::ordered,
                mutate_func: self.mutate_func,
                select_func: self.select_func,
            }
        }
    }
}