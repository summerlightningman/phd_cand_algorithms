use std::cmp::Ordering;

pub type CollectFunc<T> = Box<Fn(&[T])>;
pub type CalculationResult<T> = (T, f64);

#[derive(Clone, Copy)]
pub enum Purpose {
    Min,
    Max,
}

// impl Purpose {
//     pub fn compare(&self, a: f64, b: f64) -> Ordering {
//         match self {
//             Purpose::Min => a.partial_cmp(&b).unwrap_or(Ordering::Equal),
//             Purpose::Max => b.partial_cmp(&a).unwrap_or(Ordering::Equal),
//         }
//     }
// }
