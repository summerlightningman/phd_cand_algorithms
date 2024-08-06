pub type City = usize;
pub type Matrix = Vec<Vec<f64>>;
pub type TimeMatrix = Vec<Vec<usize>>;

pub type RuleStr = String;
pub type RuleFn = Box<dyn Fn(&Vec<City>) -> Option<i64>>;