use crate::algorithms::types::{FitnessFuncRaw};
use crate::problems::travelling_salesman::rules::{apply_rules, Rule};
use super::types::{Matrix, City};

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

pub fn calculate_distance_with_rules(matrix: Matrix, rules: Vec<Rule>) -> FitnessFuncRaw<City> {
    Box::new(move |cities: &Vec<City>| {
        let penalty: f64 = if rules.is_empty() {
            0.
        } else {
            match apply_rules(cities, &rules) {
                None => return None,
                Some(p) => p as f64
            }
        };

        Some(calculate_distance(&matrix, &cities) + penalty)
    })
}