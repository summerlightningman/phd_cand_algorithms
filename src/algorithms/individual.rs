use crate::algorithms::types::{Fitness, FitnessFuncs};

#[derive(Clone, Debug)]
pub struct Individual<T> {
    pub value: Vec<T>,
    pub fitnesses: Vec<Fitness>,
    pub fitness: Fitness,
}

impl<T> Individual<T> {
    pub fn with_fitnesses(value: Vec<T>, fitness_funcs: &FitnessFuncs<T>) -> Self {
        let fitnesses = fitness_funcs.iter().map(|func| func(&value)).collect();

        Self {
            value,
            fitnesses,
            fitness: None,
        }
    }
}