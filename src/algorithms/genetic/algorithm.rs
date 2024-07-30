use crate::algorithms::genetic::individual::Individual;
use crate::algorithms::genetic::types::{CrossoverFunc, FitnessFuncRaw, GenerateFuncRaw, Population};
use levenshtein::levenshtein;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use crate::algorithms::types::{Purpose};
use super::helpers::compare_by_fitness;

pub struct GeneticAlgorithm<T> {
    pub fitness_func: FitnessFuncRaw<T>,
    pub actors_count: usize,
    pub iters_count: u64,
    pub solutions_count: usize,
    pub p_mutation: f32,
    pub crossover_func: CrossoverFunc<T>,
    pub mutate_func: Box<dyn Fn(Vec<T>, &mut ThreadRng) -> Vec<T>>,
    pub select_func: Box<dyn Fn(Population<T>, &Purpose, &mut ThreadRng) -> Population<T>>,
    pub generate_func: GenerateFuncRaw<T>,
    pub purpose: Purpose,
}

impl<T: std::fmt::Debug + Clone + Send + Sync> GeneticAlgorithm<T> {
    pub fn run(&self) -> Result<Population<T>, &str> {
        let mut rng = thread_rng();
        let mut population: Population<T> = Vec::with_capacity(self.actors_count);

        for _ in 0..self.actors_count {
            let value = (self.generate_func)(&mut rng);
            if let Some(fitness) = (self.fitness_func)(&value) {
                population.push(Individual {
                    value,
                    fitness: Some(fitness)
                });
            }
        }

        for _ in 0..self.iters_count {
            // SELECTION
            population = (self.select_func)(population, &self.purpose, &mut rng);
            let mut new_population: Population<T> = Vec::with_capacity(self.actors_count);

            // CROSSOVER
            for individual in &population {
                let panmixia = |ind: &Individual<T>| levenshtein(&format!("{:?}", &individual.value), &format!("{:?}", ind.value));
                let partner = population.iter().max_by_key(|ind| panmixia(*ind)).unwrap();

                let (child_1, child_2) = (self.crossover_func)(individual, partner, &mut rng);

                // MUTATION
                let child_1_value = if rng.gen::<f32>() < self.p_mutation {
                    (self.mutate_func)(child_1.value, &mut rng)
                } else {
                    child_1.value
                };

                let child_2_value = if rng.gen::<f32>() < self.p_mutation {
                    (self.mutate_func)(child_2.value, &mut rng)
                } else {
                    child_2.value
                };

                if let Some(child_1_fitness) = (self.fitness_func)(&child_1_value) {
                    new_population.push(Individual {
                        value: child_1_value,
                        fitness: Some(child_1_fitness),
                    });
                }

                if let Some(child_2_fitness) = (self.fitness_func)(&child_2_value) {
                    new_population.push(Individual {
                        value: child_2_value,
                        fitness: Some(child_2_fitness),
                    });
                }
            }

            population.extend(new_population);
            population.sort_by(compare_by_fitness(&self.purpose));
            population.truncate(self.actors_count);
        }

        population.dedup_by(|a, b| {
            let fitness_a = match a.fitness {
                Some(fit) => fit,
                None => return true
            };

            let fitness_b = match b.fitness {
                Some(fit) => fit,
                None => return true
            };

            return fitness_a.total_cmp(&fitness_b).is_eq()
        });
        population.sort_by(compare_by_fitness(&self.purpose));
        population.truncate(self.solutions_count);
        population.shrink_to_fit();
        Ok(population)
    }
}
