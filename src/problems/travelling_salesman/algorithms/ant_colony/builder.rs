use std::cell::RefCell;
use std::num::NonZeroUsize;
use lru::LruCache;
use crate::algorithms::ant_colony::algorithm::AntColonyAlgorithm;
use crate::algorithms::constants::{ACTORS_COUNT, ITERS_COUNT, SOLUTIONS_COUNT};
use crate::problems::travelling_salesman::rules::parse_rule;
use crate::problems::travelling_salesman::types::{City, Matrix, RuleFn, RuleStr, TimeMatrix};
use super::algorithm::TSAntColonyAlgorithm;

pub struct TSAntColonyAlgorithmBuilder {
    matrix: Matrix,
    rules: Vec<RuleFn>,
    penalty_cache: RefCell<LruCache<Vec<City>, Option<f64>>>,
    time_matrix: Option<TimeMatrix>,
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,
    p: f64,
    q: f64,
    alpha: f64,
    beta: f64,
}

impl TSAntColonyAlgorithmBuilder {
    pub fn new(matrix: Matrix) -> Self {
        if matrix.len() != (matrix.iter().map(|row| row.len()).sum::<usize>() / matrix.len()) {
            panic!("Matrix is not squared")
        }

        Self {
            matrix,
            rules: Vec::new(),
            penalty_cache: RefCell::new(LruCache::new(NonZeroUsize::new(300).unwrap())),
            actors_count: ACTORS_COUNT,
            iters_count: ITERS_COUNT,
            solutions_count: SOLUTIONS_COUNT,
            time_matrix: None,
            p: 1.,
            q: 1.,
            alpha: 1.,
            beta: 1.
        }
    }

    pub fn actors_count(mut self, actors_count: usize) -> Self {
        self.actors_count = actors_count;
        self
    }

    pub fn iters_count(mut self, iters_count: usize) -> Self {
        self.iters_count = iters_count;
        self
    }

    pub fn solutions_count(mut self, solutions_count: usize) -> Self {
        self.solutions_count = solutions_count;
        self
    }

    pub fn p(mut self, p: f64) -> Self {
        self.p = p;
        self
    }

    pub fn q(mut self, q: f64) -> Self {
        self.q = q;
        self
    }

    pub fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        self
    }

    pub fn beta(mut self, beta: f64) -> Self {
        self.beta = beta;
        self
    }

    pub fn time_matrix(mut self, time_matrix: TimeMatrix) -> Self {
        if time_matrix.len() != self.matrix.len() {
            panic!("Time matrix size is not equal distance matrix")
        }

        self.time_matrix = Some(time_matrix);
        self
    }

    pub fn rules(mut self, rules: Vec<RuleStr>) -> Self {
        self.rules = rules.into_iter().map(|rule_str| {
            parse_rule(rule_str, self.matrix.clone(), self.time_matrix.clone())
        }).collect();

        self
    }

    pub fn build(self) -> TSAntColonyAlgorithm {
        TSAntColonyAlgorithm {
            rules: self.rules,
            penalty_cache: self.penalty_cache,
            time_matrix: self.time_matrix,
            algo: AntColonyAlgorithm {
                matrix: self.matrix,
                solutions_count: self.solutions_count,
                iters_count: self.iters_count,
                actors_count: self.actors_count,
                q: self.q,
                p: self.p,
                alpha: self.alpha,
                beta: self.beta
            }
        }
    }
}