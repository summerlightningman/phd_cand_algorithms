use std::cmp::Ordering;
use crate::algorithms::bee_colony::bee::Bee;
use crate::algorithms::types::Purpose;

pub fn compare_by_fitness<T>(purpose: &Purpose) -> impl Fn(&Bee<T>, &Bee<T>) -> Ordering + '_ {
    let stub = match purpose {
        Purpose::Min => Ordering::Greater,
        Purpose::Max => Ordering::Less,
    };

    return move |a: &Bee<T>, b: &Bee<T>| -> Ordering {
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