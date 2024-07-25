use crate::algorithms::genetic::individual::Individual;

pub type FitnessFunc<T> = fn(&Vec<T>) -> f64;
pub type CrossoverFunc<T> = for<'a> fn(&'a Individual<T>, &'a Individual<T>) -> (Individual<T>, Individual<T>);
pub type MutateFunc<T> = fn(&Vec<T>) -> Vec<T>;
pub type SelectFunc<T> = fn(Vec<Individual<T>>) -> Vec<Individual<T>>;
pub type GenerateFunc<T> = fn() -> Vec<T>;

pub type Population<T> = Vec<Individual<T>>;

