use crate::algorithms::types::{FitnessFuncRaw};
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