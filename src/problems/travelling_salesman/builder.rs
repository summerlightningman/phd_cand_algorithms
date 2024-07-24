use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;
use crate::algorithms::constants::{ACTORS_COUNT, ITERS_COUNT, SOLUTIONS_COUNT};
use crate::algorithms::genetic::algorithm::GeneticAlgorithm;
use crate::algorithms::genetic::individual::Individual;
use crate::algorithms::genetic::types::{CrossoverFunc, FitnessFunc, GenerateFunc, MutateFunc, SelectFunc};
use crate::problems::travelling_salesman::algorithms::genetic::algorithm::TSGeneticAlgorithm;
use crate::problems::travelling_salesman::helpers;
use crate::problems::travelling_salesman::types::{City, Matrix};
use rand::{seq::SliceRandom, thread_rng};
use crate::algorithms::types::Purpose;

pub struct TSGeneticAlgorithmBuilder {
    matrix: Matrix,
    actors_count: usize,
    iters_count: u64,
    solutions_count: u64,
    p_mutation: f32,
    crossover_func: CrossoverFunc<City>,
    mutate_func: MutateFunc<City>,
    select_func: SelectFunc<City>,
}

impl<T> OptimizationAlgorithmBuilder for TSGeneticAlgorithmBuilder {
    fn iters_count(mut self, iters_count: u64) -> Self {
        self.iters_count = iters_count;
        self
    }

    fn actors_count(mut self, actors_count: usize) -> Self {
        self.actors_count = actors_count;
        self
    }

    fn solutions_count(mut self, solutions_count: u64) -> Self {
        self.solutions_count = solutions_count;
        self
    }
}

impl TSGeneticAlgorithmBuilder {
    fn new(
        matrix: Matrix,
        crossover_func: CrossoverFunc<City>,
        mutate_func: MutateFunc<City>,
        select_func: SelectFunc<City>,
    ) -> Self {
        Self {
            matrix,
            crossover_func,
            mutate_func,
            select_func,
            actors_count: ACTORS_COUNT,
            solutions_count: SOLUTIONS_COUNT,
            iters_count: ITERS_COUNT,
            p_mutation: 0.3,
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

    fn build(self) -> TSGeneticAlgorithm {
        let fitness_func: FitnessFunc<City> = |cities| {
            helpers::calculate_distance(&self.matrix, cities)
        };

        let generate_func: GenerateFunc<City> = || -> Individual<City> {
            let cities_count = self.matrix.len();
            let mut rng = thread_rng();
            (0..cities_count).collect().shuffle(&mut rng);
        };

        return TSGeneticAlgorithm {
            matrix,
            algo: GeneticAlgorithm {
                fitness_func,
                generate_func,
                purpose: Purpose::Min,
                actors_count: self.actors_count,
                iters_count: self.iters_count,
                solutions_count: self.solutions_count,
                p_mutation: self.p_mutation,
                crossover_func: self.crossover_func,
                mutate_func: self.mutate_func,
                select_func: self.select_func,
            }
        }
    }
}