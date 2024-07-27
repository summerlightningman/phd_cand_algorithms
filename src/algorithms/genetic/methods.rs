use super::individual::Individual;
use super::helpers;
use rand::{Rng, thread_rng, seq::IteratorRandom};
use crate::algorithms::genetic::types::Population;
use crate::algorithms::types::Purpose;

pub struct Crossover;
pub struct Select;
pub struct Mutate;

impl Crossover {
    pub fn one_point<T: Clone>(point_idx: Option<usize>) -> impl Fn(&Individual<T>, &Individual<T>) -> (Individual<T>, Individual<T>) {
        return move |a: &Individual<T>, b: &Individual<T>| {
            let idx = if let Some(val) = point_idx {
                val
            } else {
                let mut rng = rand::thread_rng();
                rng.gen_range(0..a.value.len() - 1)
            };

            let mut values_left: Vec<T> = Vec::new();
            values_left.extend_from_slice(&a.value[..idx]);
            values_left.extend_from_slice(&b.value[idx..]);

            let mut values_right: Vec<T> = Vec::new();
            values_right.extend_from_slice(&b.value[..idx]);
            values_right.extend_from_slice(&a.value[idx..]);


            (
                Individual::new(values_left, None),
                Individual::new(values_right, None)
            )
        };
    }

    pub fn two_points<T: Copy>(points_range: (Option<usize>, Option<usize>)) -> impl Fn(&Individual<T>, &Individual<T>) -> (Individual<T>, Individual<T>) {
        return move |a: &Individual<T>, b: &Individual<T>| {
            let (point_left, point_right) = helpers::process_two_points_or_generate(a.value.len(), points_range);
            let mut values_left: Vec<T> = Vec::new();
            values_left.extend_from_slice(&a.value[..point_left]);
            values_left.extend_from_slice(&b.value[point_left..point_right]);
            values_left.extend_from_slice(&a.value[point_right..]);

            let mut values_right: Vec<T> = Vec::new();
            values_right.extend_from_slice(&b.value[..point_left]);
            values_right.extend_from_slice(&a.value[point_left..point_right]);
            values_right.extend_from_slice(&b.value[point_right..]);

            (
                Individual::new(values_left, None),
                Individual::new(values_right, None),
            )
        };
    }

    pub fn ordered<T: Clone + std::cmp::PartialEq>(a: &Individual<T>, b: &Individual<T>) -> (Individual<T>, Individual<T>) {
        let (point_left, point_right) = helpers::process_two_points_or_generate(a.value.len(), (None, None));
        let value_length = a.value.len();

        let mut child_a_value = vec![None; point_left];
        child_a_value.extend(b.value[point_left..point_right].iter().cloned().map(Some));
        child_a_value.extend(vec![None; value_length - point_right]);

        let mut child_b_value = vec![None; point_left];
        child_b_value.extend(a.value[point_left..point_right].iter().cloned().map(Some));
        child_b_value.extend(vec![None; value_length - point_right]);

        let run = |ind: &Individual<T>, arr: &mut Vec<Option<T>>| {
            let arr_len = arr.len();
            let mut curr_idx = point_right % arr_len;
            let mut arr_idx = 0;
            loop {
                if arr.iter().all(|el| el.is_some()) {
                    break;
                }
                // if arr.contains(Some(*ind.value[arr_idx])) {
                if arr.iter().flatten().any(|el| ind.value[arr_idx] == *el) {
                    arr_idx = (arr_idx + 1) % arr_len;
                } else {
                    arr[curr_idx] = Some(ind.value[arr_idx].clone());
                    curr_idx = (curr_idx + 1) % arr_len;
                    arr_idx = (arr_idx + 1) % arr_len
                }
            }
        };

        run(a, &mut child_a_value);
        run(b, &mut child_b_value);

        (
            Individual::new(child_a_value.into_iter().filter_map(|el| el).collect(), None),
            Individual::new(child_b_value.into_iter().filter_map(|el| el).collect(), None)
        )
    }
}

impl Mutate {
    pub fn swap_indexes<T: std::clone::Clone>(offset: Option<usize>) -> impl Fn(&Vec<T>) -> Vec<T> {
        move |value| {
            let (left, right) = helpers::generate_two_points(offset, value.len());
            let mut value_new = value.to_vec();

            value_new.swap(left, right);
            value_new
        }
    }

    pub fn reverse_elements<T: std::clone::Clone>(offset: Option<usize>) -> impl Fn(&Vec<T>) -> Vec<T> {
        move |value| {
            let (left, right) = helpers::generate_two_points(offset, value.len());
            let mut value_new = value.to_vec();

            value_new[left..right].reverse();
            value_new
        }
    }
}

const RATE_DEFAULT: f32 = 0.7;
impl Select {
    pub fn roulette<T: Clone>(rate: Option<f32>) -> impl Fn(Population<T>, &Purpose) -> Population<T> {
        move |population: Population<T>, purpose: &Purpose| {
            let count = helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
            let fitness_sum: f64 = population.iter().filter_map(|ind| ind.fitness).sum();
            let probabilities: Vec<f32> = population.iter().map(|ind| {
                if let Some(fitness) = ind.fitness {
                    return if let Purpose::Min = purpose {
                        (1. - fitness / fitness_sum) as f32
                    } else {
                        (fitness / fitness_sum) as f32
                    }
                } else {
                    0.
                }
            }).collect();
            helpers::weighted_random_sampling(&population, probabilities, count)
        }
    }

    pub fn stochastic<T: Clone>(rate: Option<f32>) -> impl Fn(Population<T>, &Purpose) -> Population<T> {
        move |population: Population<T>, _: &Purpose| {
            let count = helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
            let mut rng = thread_rng();
            population.into_iter().choose_multiple(&mut rng, count)
        }
    }

    pub fn tournament<T: Clone>(size: usize, rate: Option<f32>) -> impl Fn(Population<T>, &Purpose) -> Population<T> {
        move |population: Population<T>, purpose: &Purpose| {
            let count = helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
            let mut rng = thread_rng();
            let mut population_new: Population<T> = Vec::new();

            for _ in 0..count {
                let candidates: Vec<Individual<T>> = population.iter().choose_multiple(&mut rng, size).into_iter().cloned().collect();
                let winner = match purpose {
                    Purpose::Min => candidates.into_iter().min_by(helpers::compare_by_fitness(purpose)),
                    Purpose::Max => candidates.into_iter().max_by(helpers::compare_by_fitness(purpose)),
                };

                if let Some(winner) = winner {
                    population_new.push(winner);
                }
            }

            population_new
        }
    }

    pub fn best_n<T: Clone>(rate: Option<f32>)-> impl Fn(Population<T>, &Purpose) -> Population<T> {
        move |mut population: Population<T>, purpose: &Purpose| {
            let count = helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
            population.sort_by(helpers::compare_by_fitness(purpose));
            population.truncate(count);
            population
        }
    }
}