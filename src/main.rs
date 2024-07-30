mod algorithms;
mod problems;

use algorithms::ant_colony::builder::AntColonyAlgorithmBuilder;
use algorithms::algorithm::OptimizationAlgorithm;
use std::time::{Instant};
use problems::travelling_salesman::algorithms::genetic::builder::TSGeneticAlgorithmBuilder;
use algorithms::genetic::methods::{Mutate, Select};
use crate::algorithms::algorithm::OptimizationAlgorithmBuilder;
use crate::problems::travelling_salesman::rules;

fn main() {
    let matrix = vec![
        vec![0.0, 29.0, 20.0, 21.0, 16.0, 31.0, 100.0, 12.0, 4.0, 31.0],
        vec![29.0, 0.0, 15.0, 29.0, 28.0, 40.0, 72.0, 21.0, 29.0, 41.0],
        vec![20.0, 15.0, 0.0, 15.0, 14.0, 25.0, 81.0, 9.0, 23.0, 27.0],
        vec![21.0, 29.0, 15.0, 0.0, 4.0, 12.0, 92.0, 12.0, 25.0, 13.0],
        vec![16.0, 28.0, 14.0, 4.0, 0.0, 16.0, 94.0, 9.0, 20.0, 16.0],
        vec![31.0, 40.0, 25.0, 12.0, 16.0, 0.0, 95.0, 24.0, 36.0, 3.0],
        vec![100.0, 72.0, 81.0, 92.0, 94.0, 95.0, 0.0, 90.0, 101.0, 99.0],
        vec![12.0, 21.0, 9.0, 12.0, 9.0, 24.0, 90.0, 0.0, 15.0, 25.0],
        vec![4.0, 29.0, 23.0, 25.0, 20.0, 36.0, 101.0, 15.0, 0.0, 35.0],
        vec![31.0, 41.0, 27.0, 13.0, 16.0, 3.0, 99.0, 25.0, 35.0, 0.0],
    ];
    //
    let ac = AntColonyAlgorithmBuilder::new(matrix.clone())
        .iters_count(1000)
        .build();

    let ac_time_start = Instant::now();
    let ac_solutions = ac.run();
    let ac_d_time = ac_time_start.elapsed();

    println!("{:?}", ac_d_time);
    println!("{:?}", ac_solutions.unwrap());

    let ga = TSGeneticAlgorithmBuilder::new(
        matrix.clone(),
        Mutate::swap_indexes(Some(3)),
        Select::best_n(Some(0.7)),
    ).solutions_count(20).build();

    let ga_time_start = Instant::now();
    let ga_solutions = ga.run().unwrap();
    let ga_d_time = ga_time_start.elapsed();
    //
    println!("{:?}", ga_d_time);
    println!("{:?}", ga_solutions);
}
