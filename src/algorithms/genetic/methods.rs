use super::types::{CrossoverFunc, MutateFunc, SelectFunc};
use crate::algorithms::genetic::types::Population;
use crate::algorithms::helpers;
use crate::algorithms::individual::Individual;
use crate::algorithms::types::Purpose;
use rand::prelude::ThreadRng;
use rand::{seq::IteratorRandom, Rng};
use std::fmt::Debug;

pub struct Crossover;
pub struct Select;
pub struct Mutate;

impl Crossover {
    pub fn one_point<T: Clone>(point_idx: Option<usize>) -> CrossoverFunc<T> {
        CrossoverFunc(Box::new(
            move |a: &Individual<T>, b: &Individual<T>, rng: &mut ThreadRng| {
                let idx = if let Some(val) = point_idx {
                    val
                } else if a.value.len() < 1 {
                    0
                } else {
                    rng.gen_range(0..a.value.len() - 1)
                };

                let ((a_left, a_right), (b_left, b_right)) =
                    (a.value.split_at(idx), b.value.split_at(idx));
                let mut child_a_value = a_left.to_vec();
                child_a_value.extend(b_right.to_vec());

                let mut child_b_value = b_left.to_vec();
                child_b_value.extend(a_right.to_vec());

                (child_a_value, child_b_value)
            },
        ))
    }

    pub fn two_points<T: Copy>(points_range: (Option<usize>, Option<usize>)) -> CrossoverFunc<T> {
        CrossoverFunc(Box::new(
            move |a: &Individual<T>, b: &Individual<T>, rng: &mut ThreadRng| {
                let (point_left, point_right) =
                    helpers::process_two_points_or_generate(a.value.len(), points_range, rng);
                let mut values_left: Vec<T> = Vec::with_capacity(a.value.len());
                values_left.extend_from_slice(&a.value[..point_left]);
                values_left.extend_from_slice(&b.value[point_left..point_right]);
                values_left.extend_from_slice(&a.value[point_right..]);

                let mut values_right: Vec<T> = Vec::with_capacity(a.value.len());
                values_right.extend_from_slice(&b.value[..point_left]);
                values_right.extend_from_slice(&a.value[point_left..point_right]);
                values_right.extend_from_slice(&b.value[point_right..]);

                (values_left, values_right)
            },
        ))
    }

    pub fn ordered<T: Clone + PartialEq>() -> CrossoverFunc<T> {
        CrossoverFunc(Box::new(
            move |a: &Individual<T>, b: &Individual<T>, rng: &mut ThreadRng| {
                let (point_left, point_right) =
                    helpers::process_two_points_or_generate(a.value.len(), (None, None), rng);
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
                    child_a_value
                        .into_iter()
                        .filter_map(|i| Some(i).unwrap())
                        .collect(),
                    child_b_value
                        .into_iter()
                        .filter_map(|i| Some(i.unwrap()))
                        .collect(),
                )
            },
        ))
    }
}

impl Mutate {
    pub fn swap_indexes<T: Clone>(offset: Option<usize>) -> MutateFunc<T> {
        MutateFunc(Box::new(move |mut value, rng: &mut ThreadRng| {
            let (left, right) = helpers::generate_two_points(offset, value.len(), rng);
            value.swap(left, right);
            value
        }))
    }

    pub fn reverse_elements<T: Clone>(offset: Option<usize>) -> MutateFunc<T> {
        MutateFunc(Box::new(move |mut value, rng: &mut ThreadRng| {
            let (left, right) = helpers::generate_two_points(offset, value.len(), rng);
            value[left..right].reverse();
            value
        }))
    }
}

const RATE_DEFAULT: f32 = 0.7;
impl Select {
    pub fn roulette<T: Clone + Debug>(rate: Option<f32>) -> SelectFunc<T> {
        SelectFunc(Box::new(
            move |population: Population<T>, purpose: &Purpose, rng: &mut ThreadRng| {
                let count =
                    helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
                let fitness_sum: f32 = population.iter().filter_map(|ind| ind.fitness).sum();
                let probabilities: Vec<f32> = population
                    .iter()
                    .map(|ind| {
                        if let Some(fitness) = ind.fitness {
                            return if let Purpose::Min = purpose {
                                1. - fitness / fitness_sum
                            } else {
                                fitness / fitness_sum
                            };
                        } else {
                            0.
                        }
                    })
                    .collect();

                match helpers::weighted_random_sampling(&population, probabilities, count, rng) {
                    Ok(population) => population,
                    _ => Vec::new(),
                }
            },
        ))
    }

    pub fn stochastic<T: Clone>(rate: Option<f32>) -> SelectFunc<T> {
        SelectFunc(Box::new(
            move |population: Population<T>, _: &Purpose, rng: &mut ThreadRng| {
                let count =
                    helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
                population.into_iter().choose_multiple(rng, count)
            },
        ))
    }

    pub fn tournament<T: Clone>(size: usize, rate: Option<f32>) -> SelectFunc<T> {
        SelectFunc(Box::new(
            move |population: Population<T>, purpose: &Purpose, rng: &mut ThreadRng| {
                let count =
                    helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
                let mut population_new: Population<T> = Vec::with_capacity(population.len());

                for _ in 0..count {
                    let candidates: Vec<Individual<T>> = population
                        .iter()
                        .choose_multiple(rng, size)
                        .into_iter()
                        .cloned()
                        .collect();
                    let winner = match purpose {
                        Purpose::Min => candidates
                            .into_iter()
                            .min_by(helpers::compare_by_fitness(purpose)),
                        Purpose::Max => candidates
                            .into_iter()
                            .max_by(helpers::compare_by_fitness(purpose)),
                    };

                    if let Some(winner) = winner {
                        population_new.push(winner);
                    }
                }

                population_new
            },
        ))
    }

    pub fn best_n<T: Clone>(rate: Option<f32>) -> SelectFunc<T> {
        SelectFunc(Box::new(
            move |mut population: Population<T>, purpose: &Purpose, _: &mut ThreadRng| {
                let count =
                    helpers::get_count_by_rate::<T>(population.len(), rate.unwrap_or(RATE_DEFAULT));
                population.sort_by(helpers::compare_by_fitness(purpose));
                population.truncate(count);
                population
            },
        ))
    }
}
