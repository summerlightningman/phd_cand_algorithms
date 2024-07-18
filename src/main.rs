mod algorithms;

use algorithms::ant_colony::builder::AntColonyAlgorithmBuilder;

fn main() {
    let algo = AntColonyAlgorithmBuilder::new(vec![vec![0.0, 0.0], vec![0.0, 0.0]])
        .iters_count(1000)
        .build();

    println!("{:?}", algo);
}
