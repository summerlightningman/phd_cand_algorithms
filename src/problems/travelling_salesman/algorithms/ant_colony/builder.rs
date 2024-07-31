use std::cell::RefCell;
use std::num::NonZeroUsize;
use lru::LruCache;
use crate::algorithms::ant_colony::algorithm::AntColonyAlgorithm;
use crate::algorithms::constants::{ACTORS_COUNT, ITERS_COUNT, SOLUTIONS_COUNT};
use crate::problems::travelling_salesman::rules::Rule;
use crate::problems::travelling_salesman::types::{City, Matrix};
use super::algorithm::TSAntColonyAlgorithm;

pub struct TSAntColonyAlgorithmBuilder {
    matrix: Matrix,
    rules: Vec<Rule>,
    penalty_cache: RefCell<LruCache<Vec<City>, Option<f64>>>,
    actors_count: usize,
    iters_count: u64,
    solutions_count: usize,
    p: f64,
    q: f64,
    alpha: f64,
    beta: f64,
}

impl TSAntColonyAlgorithmBuilder {
    fn new(matrix: Matrix) -> Self {
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
            p: 1.,
            q: 1.,
            alpha: 1.,
            beta: 1.
        }
    }

    fn actors_count(mut self, actors_count: usize) -> Self {
        self.actors_count = actors_count;
        self
    }

    fn iters_count(mut self, iters_count: u64) -> Self {
        self.iters_count = iters_count;
        self
    }

    fn solutions_count(mut self, solutions_count: usize) -> Self {
        self.solutions_count = solutions_count;
        self
    }

    fn rules(mut self, rules: Vec<Rule>) -> Self {
        self.rules = rules;
        self
    }

    fn p(mut self, p: f64) -> Self {
        self.p = p;
        self
    }

    fn q(mut self, q: f64) -> Self {
        self.q = q;
        self
    }

    fn alpha(mut self, alpha: f64) -> Self {
        self.alpha = alpha;
        self
    }

    fn beta(mut self, beta: f64) -> Self {
        self.beta = beta;
        self
    }

    fn build(self) -> TSAntColonyAlgorithm {
        TSAntColonyAlgorithm {
            rules: self.rules,
            penalty_cache: self.penalty_cache,
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