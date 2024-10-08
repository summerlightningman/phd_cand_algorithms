mod algorithms;
mod problems;

use std::time::{Instant};
use crate::algorithms::bee_colony::research_methods;
use crate::algorithms::genetic::methods::{Mutate, Select};
use crate::algorithms::genetic::types::{MutateFunc, SelectFunc};
use crate::problems::travelling_salesman::algorithms::ant_colony::builder::TSAntColonyAlgorithmBuilder;
use crate::problems::travelling_salesman::algorithms::bee_colony::builder::TSBeeColonyAlgorithmBuilder;
use crate::problems::travelling_salesman::algorithms::genetic::builder::TSGeneticAlgorithmBuilder;

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

    let time_matrix = vec![
        vec![0, 45, 30, 12, 18, 67, 23, 56, 41, 39],
        vec![45, 0, 50, 28, 37, 52, 19, 33, 60, 25],
        vec![30, 50, 0, 15, 63, 42, 71, 44, 22, 58],
        vec![12, 28, 15, 0, 36, 54, 20, 47, 65, 31],
        vec![18, 37, 63, 36, 0, 29, 48, 59, 11, 70],
        vec![67, 52, 42, 54, 29, 0, 35, 16, 62, 40],
        vec![23, 19, 71, 20, 48, 35, 0, 53, 26, 64],
        vec![56, 33, 44, 47, 59, 16, 53, 0, 38, 21],
        vec![41, 60, 22, 65, 11, 62, 26, 38, 0, 57],
        vec![39, 25, 58, 31, 70, 40, 64, 21, 57, 0],
    ];

    let ac = TSAntColonyAlgorithmBuilder::new(matrix.clone())
        .time_matrix(time_matrix)
        // .rules(vec![
        //     "8 следует за 0 : -300".to_string(),
        //     "3 следует за 0 : -300".to_string(),
        //     "2 следует за 0 : 100".to_string(),
        // ])
        .build();
    //
    let ac_time_start = Instant::now();
    let ac_solutions = ac.run();
    let ac_d_time = ac_time_start.elapsed();
    //
    println!("{:?}", ac_d_time);
    println!("{:?}", ac_solutions.unwrap());

    // let ga = TSGeneticAlgorithmBuilder::new(
    //     matrix.clone(),
    //     Mutate::swap_indexes(Some(3)),
    //     Select::tournament(5, Some(0.7)),
    // )
    //     .time_matrix(time_matrix.clone())
    //     .rules(vec![
    //         "8 следует за 0 : -300".to_string(),
    //         "3 следует за 0 : -300".to_string(),
    //         "2 следует за 0 : 100".to_string(),
    //     ])
    //     .build();
    //
    // let ga_time_start = Instant::now();
    // let ga_solutions = ga.run().unwrap();
    // let ga_d_time = ga_time_start.elapsed();
    //
    // println!("{:?}", ga_d_time);
    // println!("{:?}", ga_solutions);

    // let bc = TSBeeColonyAlgorithmBuilder::new(
    //     matrix,
    //     research_methods::swap_indexes(Some(3))
    // )
    //     .solutions_count(20)
    //     .time_matrix(time_matrix)
    //     .rules(vec![
    //         "8 следует за 0 : -300".to_string(),
    //         "3 следует за 0 : -300".to_string(),
    //         "2 следует за 0 : 100".to_string(),
    //     ])
    //     .build();
    //
    // let bc_time_start = Instant::now();
    // let bc_solutions = bc.run().unwrap();
    // let bc_d_time = bc_time_start.elapsed();
    //
    // println!("{:?}", bc_d_time);
    // println!("{:?}", bc_solutions);
}
