use crate::algorithms::genetic::individual::Individual;
use crate::algorithms::genetic::types::{CrossoverFunc, FitnessFunc, GenerateFunc, MutateFunc, Population, SelectFunc};
use levenshtein::levenshtein;
use rand::{Rng, thread_rng};
use crate::algorithms::types::Purpose;

pub struct GeneticAlgorithm<T> {
    pub fitness_func: FitnessFunc<T>,
    pub actors_count: usize,
    pub iters_count: u64,
    pub solutions_count: u64,
    pub p_mutation: f32,
    pub crossover_func: CrossoverFunc<T>,
    pub mutate_func: MutateFunc<T>,
    pub select_func: SelectFunc<T>,
    pub generate_func: GenerateFunc<T>,
    pub purpose: Purpose,
}

impl<T: std::fmt::Debug + Clone> GeneticAlgorithm<T> {
    pub fn run(&self) {
        let mut population: Population<T> = (self.generate_func)().iter().map(|val: &Vec<T>| {
            Individual::new(val.to_vec(), Some((self.fitness_func)(val)))
        }).collect();


        for _ in 0..self.iters_count {
            // SELECTION
            population = (self.select_func)(population);
            let mut new_population: Population<T> = Vec::new();

            // CROSSOVER
            for individual in &population {
                let panmixia = |ind: &Individual<T>| levenshtein(&format!("{:?}", &individual.value), &format!("{:?}", ind.value));
                let partner = population.iter().max_by_key(|ind | panmixia(*ind)).unwrap();

                let (child_1, child_2) = (self.crossover_func)(individual, partner);
                new_population.push(child_1);
                new_population.push(child_2);

                let mut rng = thread_rng();
                new_population = new_population
                    .into_iter()
                    .map(|ind: Individual<T>| {
                        let value = if rng.gen::<f32>() < self.p_mutation {
                            (self.mutate_func)(&ind.value)
                        } else {
                            ind.value
                        };

                        Individual {
                            value: value.clone(),
                            fitness: Some((self.fitness_func)(&value))
                        }
                    })
                    .collect();
            }

            // MUTATION
            population.extend(new_population);
            population.sort_by(|ind_a, ind_b| ind_b.fitness.partial_cmp(&ind_a.fitness).unwrap());
            population.truncate(self.actors_count);
        }
    }


}