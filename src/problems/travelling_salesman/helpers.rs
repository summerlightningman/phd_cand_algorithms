use crate::algorithms::types::Fitness;
use crate::problems::travelling_salesman::rules::{Rule, apply_rules};
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

pub fn fitness_func(matrix: &Matrix, rules: Vec<Rule>) -> impl Fn(&Vec<City>) -> Fitness {
    return move |value: &Vec<City>| -> Fitness {
        let penalty: i32 = if rules.is_empty() {
            0
        } else {
            match apply_rules(value, &rules) {
                None => return None,
                Some(p) => p
            }
        };

        let distance = calculate_distance(&matrix, value) + penalty as f64;

        Some(0.)
    };
}