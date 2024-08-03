use crate::algorithms::individual::Individual;
use crate::algorithms::genetic::types::{CrossoverFunc, GenerateFuncRaw, MutateFuncRaw, Population, SelectFuncRaw};
use levenshtein::levenshtein;
use rand::{Rng, thread_rng};
use crate::algorithms::types::{FitnessFuncs, Purpose};
use super::helpers::compare_by_fitness;

pub struct GeneticAlgorithm<T> {
    pub fitness_funcs: FitnessFuncs<T>,
    pub actors_count: usize,
    pub iters_count: u64,
    pub solutions_count: usize,
    pub p_mutation: f32,
    pub crossover_func: CrossoverFunc<T>,
    pub mutate_func: MutateFuncRaw<T>,
    pub select_func: SelectFuncRaw<T>,
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

        self.calculate_fitnesses(&mut population);

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

                new_population.push(Individual::with_fitnesses(child_1_value, &self.fitness_funcs));
                new_population.push(Individual::with_fitnesses(child_2_value, &self.fitness_funcs));
            }

            self.calculate_fitnesses(&mut population);

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

            return fitness_a == fitness_b
        });
        population.sort_by(compare_by_fitness(&self.purpose));
        population.truncate(self.solutions_count);
        population.shrink_to_fit();
        Ok(population)
    }

    fn fitnesses_min_diff(&self, population: &Population<T>) -> (Vec<f32>, Vec<f32>) {
        let fitness_funcs_len = self.fitness_funcs.len();

        let mut min = vec![f32::MAX; fitness_funcs_len];
        let mut max = vec![f32::MIN; fitness_funcs_len];

        for idx in 0..fitness_funcs_len {
            for ind in population {
                if let Some(fitness) = ind.fitnesses[idx] {
                    if fitness < min[idx] {
                        min[idx] = fitness;
                    }
                    if fitness > max[idx] {
                        max[idx] = fitness;
                    }
                }
            }
        }

        let diff = max.iter().zip(min.iter()).map(|(max_val, min_val)| {
            max_val - min_val
        }).collect();

        (min, diff)
    }

    fn calculate_fitnesses(&self, population: &mut Population<T>) {
        let (fitnesses_min, fitnesses_diff) = self.fitnesses_min_diff(&population);

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
    }
}
