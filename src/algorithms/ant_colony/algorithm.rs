use super::ant::Ant;
use super::types::{City, PheromoneMatrix};

#[derive(Debug)]
pub struct AntColonyAlgorithm {
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,
    alpha: f64,
    beta: f64,
    q: f64,
    p: f64,
    matrix: Vec<Vec<f64>>,
    pheromone_matrix: Vec<Vec<f64>>,
    colony: Vec<Ant>,
}

impl AntColonyAlgorithm {
    pub fn new(
        iters_count: usize,
        solutions_count: usize,
        actors_count: usize,
        alpha: f64,
        beta: f64,
        q: f64,
        p: f64,
        matrix: Vec<Vec<f64>>,
    ) -> Self {
        let cities_count = matrix.len();

        Self {
            iters_count,
            solutions_count,
            actors_count,
            alpha,
            beta,
            q,
            p,
            matrix,
            pheromone_matrix: vec![vec![1.; cities_count]; cities_count],
            colony: vec![Ant::new(&cities_count); cities_count],
        }
    }

    fn get_pheromone(&self, ant: Ant, city: City) -> f64 {
        self.pheromone_matrix[ant.current_city()][city]
    }

    
}

#[derive(Debug)]
pub struct AntColonyAlgorithmBuilder {
    actors_count: usize,
    iters_count: usize,
    solutions_count: usize,
    alpha: f64,
    beta: f64,
    q: f64,
    p: f64,
    matrix: Vec<Vec<f64>>,
}

impl AntColonyAlgorithmBuilder {
    pub fn new(matrix: Vec<Vec<f64>>) -> Self {
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
