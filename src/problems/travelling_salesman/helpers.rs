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