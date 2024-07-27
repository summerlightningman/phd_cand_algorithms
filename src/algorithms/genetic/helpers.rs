use std::cmp::Ordering;
use rand::{Rng, thread_rng};
use rand::distributions::{WeightedIndex};
use crate::algorithms::genetic::individual::Individual;
use crate::algorithms::types::Purpose;
use rand::distributions::Distribution;

pub fn process_two_points_or_generate(seq_length: usize, points: (Option<usize>, Option<usize>)) -> (usize, usize) {
    let mut rnd = thread_rng();
    let (point_left, point_right) = points;
    let middle = seq_length / 2;
    let left = match point_left {
        Some(val) => val,
        _ => rnd.gen_range(0..middle)
    };
    let right = match point_right {
        Some(val) => val,
        _ => rnd.gen_range(middle..seq_length)
    };

    if left > right {
        (right, left)
    } else {
        (left, right)
    }
}

pub fn get_count_by_rate<T>(population_len: usize, rate: f32) -> usize {
    let count = (population_len as f32) * rate;
    return count.round() as usize
}

pub fn generate_two_points(offset_: Option<usize>, seq_length: usize) -> (usize, usize) {
    if seq_length <= 1 {
        return (0, 0);
    }

    let offset = match offset_ {
        Some(o) => o,
        None => {
            let mut rng = thread_rng();
            rng.gen_range(0..seq_length / 2)
        }
    };

    let mut rng = thread_rng();
    let mut a = rng.gen_range(0..seq_length);
    let mut b = if rng.gen_bool(0.5) {
        a + offset
    } else {
        a.saturating_sub(offset)
    };

    if b >= seq_length {
        b = seq_length - 1;
    }

    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

pub fn weighted_random_sampling<T: Clone>(items: &Vec<T>, weights: Vec<f32>, k: usize) -> Vec<T> {
    let mut rng = thread_rng();
    let dist = WeightedIndex::new(weights).unwrap();

    (0..k).map(|_| {
        let index = dist.sample(&mut rng);
        items[index].clone()
    }).collect()
}



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
            Purpose::Min => b_fitness.partial_cmp(&a_fitness).unwrap(),
            Purpose::Max => a_fitness.partial_cmp(&b_fitness).unwrap()
        }
    }
}