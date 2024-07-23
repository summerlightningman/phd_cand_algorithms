use super::algorithm::AntColonyAlgorithm;
use super::types::Matrix;

#[allow(dead_code)]
pub struct AntColonyAlgorithmBuilder {
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,
    alpha: f64,
    beta: f64,
    q: f64,
    p: f64,
    matrix: Matrix,
}

#[allow(dead_code)]
impl AntColonyAlgorithmBuilder {
    pub fn new(matrix: Matrix) -> Self {

        if matrix.len() != (matrix.iter().map(|row| row.len()).sum::<usize>() / matrix.len()) {
            panic!("Matrix is not squared")
        }

        Self {
            matrix,
            iters_count: 100,
            solutions_count: 3,
            actors_count: 50,
            alpha: 1.0,
            beta: 1.0,
            q: 1.0,
            p: 1.0,
        }
    }

    pub fn iters_count(mut self, count: usize) -> Self {
        self.iters_count = count;
        self
    }

    pub fn solutions_count(mut self, count: usize) -> Self {
        self.solutions_count = count;
        self
    }

    pub fn actors_count(mut self, count: usize) -> Self {
        self.actors_count = count;
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

    pub fn q(mut self, q: f64) -> Self {
        self.q = q;
        self
    }

    pub fn p(mut self, p: f64) -> Self {
        self.p = p;
        self
    }

    pub fn build(self) -> AntColonyAlgorithm {
        AntColonyAlgorithm::new(
            self.iters_count,
            self.solutions_count,
            self.actors_count,
            self.alpha,
            self.beta,
            self.q,
            self.p,
            self.matrix,
        )
    }
}
