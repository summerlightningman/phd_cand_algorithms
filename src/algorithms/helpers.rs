use rand::{Rng, thread_rng};
use rand::distributions::{WeightedIndex, Distribution};
use rand::rngs::ThreadRng;

pub fn generate_two_points(offset_: Option<usize>, seq_length: usize, rng: &mut ThreadRng) -> (usize, usize) {
    if seq_length <= 1 {
        return (0, 0);
    }

    let offset = match offset_ {
        Some(o) => o,
        None => {
            rng.gen_range(0..seq_length / 2)
        }
    };

    let a = rng.gen_range(0..seq_length);
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

pub fn process_two_points_or_generate(seq_length: usize, points: (Option<usize>, Option<usize>), rng: &mut ThreadRng) -> (usize, usize) {
    let (point_left, point_right) = points;
    let middle = seq_length / 2;
    let left = match point_left {
        Some(val) => val,
        _ => rng.gen_range(0..middle)
    };
    let right = match point_right {
        Some(val) => val,
        _ => rng.gen_range(middle..seq_length)
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

pub fn weighted_random_sampling<T: Clone>(items: &Vec<T>, weights: Vec<f32>, k: usize, rng: &mut ThreadRng) -> Vec<T> {
    let dist = WeightedIndex::new(weights).unwrap();

    (0..k).map(|_| {
        let index = dist.sample(rng);
        items[index].clone()
    }).collect()
}
