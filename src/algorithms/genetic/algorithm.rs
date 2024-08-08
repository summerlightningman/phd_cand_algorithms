use crate::algorithms::{
    individual::Individual,
    genetic::types::{CrossoverFunc, GenerateFuncRaw, MutateFuncRaw, Population, SelectFuncRaw},
    types::{FitnessFuncs, Purpose},
    helpers
};
use levenshtein::levenshtein;
use rand::{Rng, thread_rng};


pub struct GeneticAlgorithm<T> {
    pub fitness_funcs: FitnessFuncs<T>,
    pub actors_count: usize,
    pub iters_count: usize,
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
        let mut population: Population<T> = Vec::with_capacity(self.actors_count );

        for _ in 0..self.actors_count {
            let value = (self.generate_func)(&mut rng);
            population.push(Individual::with_fitnesses(value, &self.fitness_funcs));
        }

        helpers::calculate_fitnesses(&mut population, &self.fitness_funcs);

        for _ in 0..self.iters_count {
            // SELECTION
            population = (self.select_func)(population, &self.purpose, &mut rng);
            let mut new_population: Population<T> = Vec::with_capacity(self.actors_count );

            // CROSSOVER
            for individual in &population {
                let panmixia = |ind: &Individual<T>| levenshtein(&format!("{:?}", &individual.value), &format!("{:?}", ind.value));
                let partner = population.iter().max_by_key(|ind| panmixia(*ind)).unwrap();

                let (child_1, child_2) = (self.crossover_func)(individual, partner, &mut rng);

                // MUTATION
                let child_1_value = if rng.gen::<f32>() < self.p_mutation {
                    (self.mutate_func)(child_1, &mut rng)
                } else {
                    child_1
                };

                let child_2_value = if rng.gen::<f32>() < self.p_mutation {
                    (self.mutate_func)(child_2, &mut rng)
                } else {
                    child_2
                };

                new_population.push(Individual::with_fitnesses(child_1_value, &self.fitness_funcs));
                new_population.push(Individual::with_fitnesses(child_2_value, &self.fitness_funcs));
            }

            helpers::calculate_fitnesses(&mut population, &self.fitness_funcs);

            population.extend(new_population);
            population.sort_by(helpers::compare_by_fitness(&self.purpose));
            population.truncate(self.actors_count );
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
        population.sort_by(helpers::compare_by_fitness(&self.purpose));
        population.truncate(self.solutions_count );
        population.shrink_to_fit();
        Ok(population)
    }
}
