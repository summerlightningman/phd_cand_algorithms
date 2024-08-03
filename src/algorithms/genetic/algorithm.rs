use crate::algorithms::individual::Individual;
use crate::algorithms::genetic::types::{CrossoverFunc, FitnessFuncRaw, GenerateFuncRaw, Population};
use levenshtein::levenshtein;
use rand::{Rng, thread_rng};
use rand::rngs::ThreadRng;
use crate::algorithms::types::{Purpose};
use super::helpers::compare_by_fitness;

pub struct GeneticAlgorithm<T> {
    pub fitness_funcs: Vec<FitnessFuncRaw<T>>,
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
            population.push(Individual::with_fitnesses(value, &self.fitness_funcs));
        }

        let fitnesses_min: Vec<f64> = (0..self.fitness_funcs.len()).map(|idx| {
            population.iter().map(|ind| ind.fitnesses[idx]).filter(|fitness| fitness.is_some()).collect().min()
        }).collect();
        let fitnesses_max: Vec<f64> = (0..self.fitness_funcs.len()).map(|idx| {
            population.iter().map(|ind| ind.fitnesses[idx]).filter(|fitness| fitness.is_some()).collect().max()
        }).collect();
        let fitnesses_diff: Vec<f64> = (0..self.fitness_funcs.len()).map(|idx| {
           fitnesses_max[idx] - fitnesses_min[idx]
        }).collect();

        'outer: for ind in population.iter_mut() {
            let mut fitness = 0.;

            for (idx, fitness_raw) in ind.fitnesses.iter().enumerate() {
                if let Some(fitness_raw) = fitness_raw {
                    fitness += (fitness_raw - fitnesses_min[idx]) / fitnesses_diff[idx]
                } else {
                    ind.fitness = None;
                    continue 'outer
                }
            }

            ind.fitness = Some(fitness);
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

            return fitness_a.partial_cmp(&fitness_b).is_eq()
        });
        population.sort_by(compare_by_fitness(&self.purpose));
        population.truncate(self.solutions_count);
        population.shrink_to_fit();
        Ok(population)
    }
}
