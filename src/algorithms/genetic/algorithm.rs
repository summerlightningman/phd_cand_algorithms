use crate::algorithms::{
    individual::Individual,
    genetic::types::{CrossoverFunc, GenerateFunc, MutateFunc, Population, SelectFunc},
    types::{FitnessFuncs, Purpose},
    helpers
};
use rand::{Rng, thread_rng};
use rand::seq::SliceRandom;


pub struct GeneticAlgorithm<T> {
    pub fitness_funcs: FitnessFuncs<T>,
    pub actors_count: usize,
    pub iters_count: usize,
    pub solutions_count: usize,
    pub p_mutation: f32,
    pub crossover_func: CrossoverFunc<T>,
    pub mutate_func: MutateFunc<T>,
    pub select_func: SelectFunc<T>,
    pub generate_func: GenerateFunc<T>,
    pub purpose: Purpose,
}

impl<T: std::fmt::Debug + Clone + Send + Sync> GeneticAlgorithm<T> {
    pub fn run(&self) -> Result<Population<T>, &str> {
        let mut rng = thread_rng();
        let mut population: Population<T> = Vec::with_capacity(self.actors_count );

        for _ in 0..self.actors_count {
            let value = self.generate_func.0(&mut rng);
            population.push(Individual::with_fitnesses(value, &self.fitness_funcs));
        }

        helpers::calculate_fitnesses(&mut population, &self.fitness_funcs);

        for _ in 0..self.iters_count {
            // SELECTION
            if population.iter().all(|individual: &Individual<T>| {
                individual.fitness.unwrap_or(0.) == 0.
            }) {
                break
            };
            population = self.select_func.0(population, &self.purpose, &mut rng);

            // CROSSOVER
            let new_population: Population<T> = population.iter().flat_map(|individual: &Individual<T>| {
                // let panmixia = |ind: &Individual<T>| levenshtein(&format!("{:?}", &individual.value), &format!("{:?}", ind.value));
                // let partner = population.iter().max_by_key(|ind| panmixia(*ind)).unwrap();
                let partner = population.choose(&mut rng).unwrap();

                let (child_1, child_2) = self.crossover_func.0(individual, partner, &mut rng);

                // MUTATION
                let child_1_value = if rng.gen::<f32>() < self.p_mutation {
                    self.mutate_func.0(child_1, &mut rng)
                } else {
                    child_1
                };

                let child_2_value = if rng.gen::<f32>() < self.p_mutation {
                    self.mutate_func.0(child_2, &mut rng)
                } else {
                    child_2
                };

                vec![
                    Individual::with_fitnesses(child_1_value, &self.fitness_funcs),
                    Individual::with_fitnesses(child_2_value, &self.fitness_funcs)
                ]
            }).collect();

            population.extend(new_population);

            helpers::calculate_fitnesses(&mut population, &self.fitness_funcs);
            population.sort_by(helpers::compare_by_fitness(&self.purpose));
            population.truncate(self.actors_count );
        }

        population.dedup_by(|a, b| a.fitness == b.fitness);
        population.sort_unstable_by(helpers::compare_by_fitness(&self.purpose));
        population.truncate(self.solutions_count );
        Ok(population)
    }
}
