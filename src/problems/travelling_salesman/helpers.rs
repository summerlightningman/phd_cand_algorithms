use crate::algorithms::solution::Solution;
use crate::algorithms::types::{FitnessFuncRaw, FitnessFuncs, Population};
use super::types::{Matrix, City, TimeMatrix, RuleFn};

pub fn calculate_distance(matrix: &Matrix, cities: &Vec<City>) -> f64 {
    let mut sum: f64 = 0.;
    let cities_count = cities.len();
    for i in 0..cities_count {
        let j = (i + 1) % cities_count;

        let city_a = cities[i];
        let city_b = cities[j];

        sum += matrix[city_a][city_b];
    }

    sum
}

pub fn calculate_time(time_matrix: &TimeMatrix, cities: &Vec<City>) -> usize {
    let mut sum: usize = 0;
    let cities_count = cities.len();
    for i in 0..cities_count {
        let j = (i + 1) % cities_count;

        let city_a = cities[i];
        let city_b = cities[j];

        sum += time_matrix[city_a][city_b];
    }

    sum
}

pub fn calculate_distance_with_rules(matrix: Matrix, rules: Vec<RuleFn>) -> FitnessFuncRaw<City> {
    Box::new(move |cities: &Vec<City>| {
        let penalty = if rules.is_empty() {
            0.
        } else {
            let mut p = 0;
            for evaluate in rules.iter() {
                match evaluate(cities) {
                    Some(pen) => p += pen,
                    None => return None
                }
            }
            p as f64
        };

        Some(calculate_distance(&matrix, &cities) + penalty)
    })
}

pub fn time_fitness(time_matrix: Option<TimeMatrix>) -> FitnessFuncRaw<City> {
    if let Some(t_matrix) = time_matrix {
        Box::new(move |cities: &Vec<City>| {
            Some(calculate_time(&t_matrix, cities) as f64)
        })
    } else {
        Box::new(move |_| Some(0.))
    }
}

pub fn make_solutions(population: Population<City>, solutions_count: usize, fitness_funcs: &FitnessFuncs<City>) -> Vec<Solution> {
    let mut solutions: Vec<Solution> = Vec::new();

    for ind in population.into_iter() {
        if ind.fitness.is_some() {
            let distance = ind.fitnesses[0].unwrap();
            let time = if fitness_funcs.len() > 1 {
                Some(ind.fitnesses[1].unwrap() as usize)
            } else {
                None
            };

            solutions.push(Solution {
                path: ind.value,
                distance,
                time,
            });
        }

        if solutions.len() == solutions_count  {
            break
        }
    }

    solutions
}