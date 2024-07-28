pub type FoodSource<T> = Vec<T>;
pub type Fitness = f64;

pub type GenerateFuncRaw<T> = Box<dyn Fn() -> FoodSource<T>>;
pub type GenerateFunc<T> = fn() -> FoodSource<T>;

pub type FitnessFuncRaw<T> = Box<dyn Fn(&FoodSource<T>) -> Fitness>;
pub type FitnessFunc<T> = fn(&FoodSource<T>) -> Fitness;

pub type ResearchFunc<T> = fn(&Vec<T>) -> FoodSource<T>;

pub type ResearchFuncRaw<T> = Box<dyn Fn(&FoodSource<T>) -> FoodSource<T>>;