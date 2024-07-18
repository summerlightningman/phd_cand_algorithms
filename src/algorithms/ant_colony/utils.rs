use super::types::{City, Matrix};

pub fn calculate_distance(path: &Vec<City>, matrix: &Matrix) -> f64 {
    path.iter()
        .zip(path[1..].iter().chain(std::iter::once(&path[0])))
        .map(|(&a, &b)| matrix[a][b])
        .sum()
}
