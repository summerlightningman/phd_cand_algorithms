use crate::algorithms::individual::Individual;

#[derive(Clone, Copy)]
pub enum Purpose {
    Min,
    Max,
}

pub type Fitness = Option<f32>;
pub type FitnessRaw = Option<f64>;
pub type FitnessFuncRaw<T> = Box<dyn Fn(&Vec<T>) -> FitnessRaw>;
pub type FitnessFuncs<T> = Vec<FitnessFuncRaw<T>>;
pub type Population<T> = Vec<Individual<T>>;