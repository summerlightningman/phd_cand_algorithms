use crate::algorithms::genetic::individual::Individual;
use crate::algorithms::genetic::types::{CrossoverFunc, FitnessFuncRaw, GenerateFuncRaw, Population};
use levenshtein::levenshtein;
use rand::{Rng, thread_rng};
use crate::algorithms::types::{Purpose};
use super::helpers::compare_by_fitness;

pub struct GeneticAlgorithm<T> {
    pub fitness_func: FitnessFuncRaw<T>,
    pub actors_count: usize,
    pub iters_count: u64,
    pub solutions_count: usize,
    pub p_mutation: f32,
    pub crossover_func: CrossoverFunc<T>,
    pub mutate_func: Box<dyn Fn(Vec<T>) -> Vec<T>>,
    pub select_func: Box<dyn Fn(Population<T>, &Purpose) -> Population<T>>,
    pub generate_func: GenerateFuncRaw<T>,
    pub purpose: Purpose,
}

impl<T: std::fmt::Debug + Clone> GeneticAlgorithm<T> {
    pub fn run(&self) -> Result<Population<T>, &str> {
        let mut rng = thread_rng();
        let mut population: Population<T> = (0..self.actors_count).map(|_| {
            let value = (self.generate_func)();
            let fitness = (self.fitness_func)(&value);
            Individual {
                value: value,
                fitness: Some(fitness),
            }
        }).collect();

        for _ in 0..self.iters_count {
            // SELECTION
            population = (self.select_func)(population, &self.purpose);
            let mut new_population: Population<T> = Vec::new();

            // CROSSOVER
            for individual in &population {
                let panmixia = |ind: &Individual<T>| levenshtein(&format!("{:?}", &individual.value), &format!("{:?}", ind.value));
                let partner = population.iter().max_by_key(|ind| panmixia(*ind)).unwrap();

                let (child_1, child_2) = (self.crossover_func)(individual, partner);

                // MUTATION
                let child_1_value = if rng.gen::<f32>() < self.p_mutation {
                    (self.mutate_func)(child_1.value)
                } else {
                    child_1.value
                };

                let child_2_value = if rng.gen::<f32>() < self.p_mutation {
                    (self.mutate_func)(child_2.value)
                } else {
                    child_2.value
                };

                let child_1_fitness = (self.fitness_func)(&child_1_value);
                let child_2_fitness = (self.fitness_func)(&child_2_value);

                new_population.push(Individual {
                    value: child_1_value,
                    fitness: Some(child_1_fitness),
                });

                new_population.push(Individual {
                    value: child_2_value,
                    fitness: Some(child_2_fitness),
                });
            }

            population.extend(new_population);
            population.sort_by(compare_by_fitness(&self.purpose));
            population.truncate(self.actors_count);
        }

        Ok(population)
    }
}
