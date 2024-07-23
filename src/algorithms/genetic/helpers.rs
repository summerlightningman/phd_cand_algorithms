use rand::{Rng, thread_rng};
use crate::algorithms::genetic::individual::Individual;

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

pub fn get_probabilities<T>(population: &Vec<Individual<T>>) -> Vec<f64> {
    let fitness_sum: f64 = population.iter().filter_map(|ind| ind.fitness).sum();
    return population.iter().filter_map(|ind| ind.fitness).map(|fitness| fitness / fitness_sum).collect()
}

pub fn get_count_by_rate<T>(population: &Vec<Individual<T>>, rate: f64) -> f64 {
    let count = (population.len() as f64) * rate;
    return count.round()
}

pub fn generate_two_points(offset_: Option<usize>, seq_length: usize) -> (usize, usize) {
    let offset = match offset_ {
        Some(o) => o,
        None => {
            let mut rng = thread_rng();
            rng.gen_range(0..seq_length - 1)
        }
    };

    if seq_length - 1 < offset {
        return generate_two_points(Some(seq_length / 2), seq_length)
    }

    let mut rng = thread_rng();
    let mut a = rng.gen_range(0..seq_length - 1);

    let b = if a + offset < seq_length {
        a + offset
    } else if a - offset >= 0 {
        a - offset
    } else {
        a = seq_length - 1;
        a - offset
    };

    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}