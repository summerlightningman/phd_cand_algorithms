use crate::algorithms::individual::Individual;
use crate::algorithms::types::Purpose;
use rand::rngs::ThreadRng;

pub type Population<T> = Vec<Individual<T>>;

pub struct CrossoverFunc<T>(
    pub Box<dyn Fn(&Individual<T>, &Individual<T>, &mut ThreadRng) -> (Vec<T>, Vec<T>)>,
);
pub struct GenerateFunc<T>(pub Box<dyn Fn(&mut ThreadRng) -> Vec<T>>);
pub struct MutateFunc<T>(pub Box<dyn Fn(Vec<T>, &mut ThreadRng) -> Vec<T>>);
pub struct SelectFunc<T>(pub Box<dyn Fn(Population<T>, &Purpose, &mut ThreadRng) -> Population<T>>);
