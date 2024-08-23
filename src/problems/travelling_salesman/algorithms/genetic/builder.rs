use crate::algorithms::constants::{ACTORS_COUNT, ITERS_COUNT, SOLUTIONS_COUNT};
use crate::algorithms::genetic::algorithm::GeneticAlgorithm;
use crate::algorithms::genetic::types::{GenerateFunc, MutateFunc, SelectFunc};
use crate::problems::travelling_salesman::algorithms::genetic::algorithm::TSGeneticAlgorithm;
use crate::problems::travelling_salesman::types::{City, Matrix, RuleFn, RuleStr, TimeMatrix};
use rand::prelude::SliceRandom;
use rand::rngs::ThreadRng;
use crate::algorithms::genetic::methods::Crossover;
use crate::algorithms::types::{FitnessFuncRaw, Purpose};
use crate::problems::travelling_salesman::helpers::{calculate_distance_with_rules, time_fitness};
use crate::problems::travelling_salesman::rules::parse_rule;

pub struct TSGeneticAlgorithmBuilder {
    matrix: Matrix,
    time_matrix: Option<TimeMatrix>,
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,
    p_mutation: f32,
    mutate_func: MutateFunc<City>,
    select_func: SelectFunc<City>,
    rules: Vec<RuleFn>,
}

impl TSGeneticAlgorithmBuilder {
    pub fn new(
        matrix: Matrix,
        mutate_func: MutateFunc<City>,
        select_func: SelectFunc<City>,
    ) -> Self {
        Self {
            matrix,
            time_matrix: None,
            mutate_func,
            select_func,
            actors_count: ACTORS_COUNT,
            solutions_count: SOLUTIONS_COUNT,
            iters_count: ITERS_COUNT,
            rules: Vec::new(),
            p_mutation: 0.3,
        }
    }

    pub fn iters_count(mut self, iters_count: usize) -> Self {
        self.iters_count = iters_count;
        self
    }

    pub fn actors_count(mut self, actors_count: usize) -> Self {
        self.actors_count = actors_count;
        self
    }

    pub fn solutions_count(mut self, solutions_count: usize) -> Self {
        self.solutions_count = solutions_count;
        self
    }

    pub fn time_matrix(mut self, time_matrix: TimeMatrix) -> Self {
        if time_matrix.len() != self.matrix.len() {
            panic!("Time matrix size is not equal distance matrix")
        }

        self.time_matrix = Some(time_matrix);
        self
    }

    pub fn p_mutation(mut self, p_mutation: f32) -> Self {
        if p_mutation < 1. {
            self.p_mutation = p_mutation;
            self
        } else {
            panic!("Value must be 0 <= p_mutation < 1")
        }
    }

    pub fn rules(mut self, rules: Vec<RuleStr>) -> Self {
        self.rules = rules.into_iter().map(|rule_str| {
            parse_rule(rule_str, self.matrix.clone(), self.time_matrix.clone())
        }).collect();

        self
    }

    pub fn build(self) -> TSGeneticAlgorithm {
        let cities_count = self.matrix.len();

        let mut fitness_funcs = vec![
            Box::new(calculate_distance_with_rules(self.matrix, self.rules)) as FitnessFuncRaw<City>,
        ];

        if let Some(time_matrix) = self.time_matrix {
            fitness_funcs.push(time_fitness(time_matrix))
        }

        let generate_func: GenerateFunc<City> = GenerateFunc(Box::new(move |rng: &mut ThreadRng| {
            let mut value: Vec<usize> = (0..cities_count).collect();
            value.shuffle(rng);
            value
        }));

        TSGeneticAlgorithm {
            algo: GeneticAlgorithm {
                fitness_funcs,
                generate_func,
                purpose: Purpose::Min,
                actors_count: self.actors_count,
                iters_count: self.iters_count,
                solutions_count: self.solutions_count,
                p_mutation: self.p_mutation,
                crossover_func: Crossover::ordered(),
                mutate_func: self.mutate_func,
                select_func: self.select_func,
            }
        }
    }
}