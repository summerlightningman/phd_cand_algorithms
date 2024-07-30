use rand::rngs::ThreadRng;
use crate::algorithms::genetic::individual::Individual;

pub type FitnessFuncRaw<T> = Box<dyn Fn(&Vec<T>) -> Option<f64>>;
pub type FitnessFunc<T> = fn(&Vec<T>) -> Option<f64>;
pub type CrossoverFunc<T> = for<'a> fn(&'a Individual<T>, &'a Individual<T>, &mut ThreadRng) -> (Individual<T>, Individual<T>);
pub type GenerateFuncRaw<T> = Box<dyn Fn() -> Vec<T>>;
pub type GenerateFunc<T> = fn() -> Vec<T>;

pub type Population<T> = Vec<Individual<T>>;

