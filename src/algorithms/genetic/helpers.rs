use std::cmp::Ordering;
use crate::algorithms::genetic::individual::Individual;
use crate::algorithms::types::Purpose;


pub fn compare_by_fitness<T>(purpose: &Purpose) -> impl Fn(&Individual<T>, &Individual<T>) -> Ordering + '_ {
    let stub = match purpose {
        Purpose::Min => Ordering::Greater,
        Purpose::Max => Ordering::Less,
    };

    return move |a: &Individual<T>, b: &Individual<T>| -> Ordering {
        let a_fitness = match a.fitness {
            Some(fit) => fit,
            None => return stub,
        };
        let b_fitness = match b.fitness {
            Some(fit) => fit,
            None => return stub
        };

        return match purpose {
            Purpose::Min => b_fitness.total_cmp(&a_fitness),
            Purpose::Max => a_fitness.total_cmp(&b_fitness)
        }
    }
}