use rand::rngs::ThreadRng;
use crate::algorithms::individual::Individual;
use crate::algorithms::types::{Purpose};

pub type Population<T> = Vec<Individual<T>>;

pub type CrossoverFunc<T> = for<'a> fn(&'a Individual<T>, &'a Individual<T>, &mut ThreadRng) -> (Vec<T>, Vec<T>);
pub type GenerateFuncRaw<T> = Box<dyn Fn(&mut ThreadRng) -> Vec<T>>;
pub type GenerateFunc<T> = fn(&mut ThreadRng) -> Vec<T>;

pub type MutateFuncRaw<T> = Box<dyn Fn(Vec<T>, &mut ThreadRng) -> Vec<T>>;
pub type SelectFuncRaw<T> = Box<dyn Fn(Population<T>, &Purpose, &mut ThreadRng) -> Population<T>>;



