use crate::algorithms::genetic::individual::Individual;

pub type FitnessFunc<T> = fn(Vec<T>) -> f64;
pub type CrossoverFunc<T> = for<'a> fn(&'a Individual<T>, &'a Individual<T>) -> (&'a Individual<T>, &'a Individual<T>);
pub type MutateFunc<T> = fn(&Individual<T>) -> Individual<T>;
pub type SelectFunc<T> = fn(Vec<Individual<T>>) -> Vec<Individual<T>>;