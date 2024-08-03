use rand::rngs::ThreadRng;
use crate::algorithms::types::Fitness;

pub type FoodSource<T> = Vec<T>;

pub type GenerateFuncRaw<T> = Box<dyn Fn() -> FoodSource<T>>;
pub type GenerateFunc<T> = fn() -> FoodSource<T>;

pub type FitnessFuncRaw<T> = Box<dyn Fn(&FoodSource<T>) -> Option<Fitness>>;
pub type FitnessFunc<T> = fn(&FoodSource<T>) -> Option<Fitness>;

pub type ResearchFuncRaw<T> = Box<dyn Fn(&FoodSource<T>, &mut ThreadRng) -> FoodSource<T>>;