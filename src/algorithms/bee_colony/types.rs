use rand::rngs::ThreadRng;

pub type FoodSource<T> = Vec<T>;

pub type GenerateFuncRaw<T> = Box<dyn Fn() -> FoodSource<T>>;
pub struct ResearchFunction<T>(pub(crate) Box<dyn Fn(&Vec<T>, &mut ThreadRng) -> Vec<T>>);
pub type ResearchFuncRaw<T> = Box<dyn Fn(&FoodSource<T>, &mut ThreadRng) -> FoodSource<T>>;