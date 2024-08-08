use std::fmt::Debug;
use rand::thread_rng;
use random_choice::random_choice;
use crate::algorithms::types::{FitnessFuncs, Population, Purpose};
use super::types::{ResearchFuncRaw, GenerateFuncRaw};
use crate::algorithms::helpers;
use crate::algorithms::individual::Individual as Bee;

pub struct BeeColonyAlgorithm<T> {
    pub actors_count: usize,
    pub iters_count: usize,
    pub solutions_count: usize,
    pub workers_part: f32,
    pub purpose: Purpose,
    pub fitness_funcs: FitnessFuncs<T>,
    pub research_func: ResearchFuncRaw<T>,
    pub generate_func: GenerateFuncRaw<T>,
}

impl<T: Clone + Debug> BeeColonyAlgorithm<T> {
    pub fn run(&self) -> Result<Vec<Bee<T>>, &str> {
        let workers_count = (self.workers_part * self.actors_count as f32).round() as usize;
        let onlookers_count = self.actors_count - workers_count;
        let mut workers: Population<T> = (0..workers_count).map(|_| self.generate_bee()).collect();
        let mut rng = thread_rng();

        helpers::calculate_fitnesses(&mut workers, &self.fitness_funcs);

        for _ in 0..self.iters_count {
            let mut onlookers: Population<T> = (0..onlookers_count).map(|_| { self.generate_bee() }).collect();
            helpers::calculate_fitnesses(&mut onlookers, &self.fitness_funcs);
            onlookers.shrink_to_fit();

            let probabilities = self.get_source_probabilities(&onlookers);
            for worker in workers.iter_mut() {
                let selected_source: &Bee<T> = self.select_onlooker_by_probabilities(&onlookers, &probabilities);

                let researched_source = (self.research_func)(&worker.value, &mut rng);
                let researched_fitness = worker.fitness;

                if let Purpose::Min = self.purpose {
                    if researched_fitness < selected_source.fitness && (worker.fitness.is_some() && researched_fitness < worker.fitness || worker.fitness.is_none()) {
                        worker.value = researched_source;
                        worker.fitness = researched_fitness;
                    } else if selected_source.fitness.is_some() && (worker.fitness.is_some() && selected_source.fitness < worker.fitness || worker.fitness.is_none()) {
                        worker.value = selected_source.value.clone();
                        worker.fitness = selected_source.fitness;
                    }
                } else {
                    if researched_fitness.is_some() && researched_fitness > selected_source.fitness && researched_fitness > worker.fitness {
                        worker.value = researched_source;
                        worker.fitness = researched_fitness;
                    } else if selected_source.fitness.is_some() && selected_source.fitness > worker.fitness {
                        worker.value = selected_source.value.clone();
                        worker.fitness = selected_source.fitness;
                    }
                }
            }
        }

        helpers::calculate_fitnesses(&mut workers, &self.fitness_funcs);

        workers.dedup_by(|a, b| {
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
        workers.sort_by(helpers::compare_by_fitness(&self.purpose));
        workers.truncate(self.solutions_count );
        Ok(workers)
    }

    fn generate_bee(&self) -> Bee<T> {
        let source = (self.generate_func)();
        Bee::with_fitnesses(source, &self.fitness_funcs)
    }

    fn get_source_probabilities(&self, onlookers: &Vec<Bee<T>>) -> Vec<f32> {
        let fitness_sum: f32 = onlookers.iter().filter_map(|bee| bee.fitness).sum();
        let get_probability: fn(Option<f32>, f32) -> f32 = match self.purpose {
            Purpose::Min => |fit: Option<f32>, sum: f32| {
                if let Some(fitness) = fit {
                    1. - fitness / sum
                } else {
                    0.
                }
            },
            Purpose::Max => |fit: Option<f32>, sum: f32| {
                if let Some(fitness) = fit {
                    fitness / sum
                } else {
                    0.
                }
            }
        };
        return onlookers.iter().map(|bee| {
            get_probability(bee.fitness, fitness_sum)
        }).collect();
    }

    fn select_onlooker_by_probabilities<'a>(&self, onlookers: &'a Vec<Bee<T>>, probabilities: &Vec<f32>) -> &'a Bee<T> {
        let selected_bees = random_choice().random_choice_f32(&onlookers, &probabilities, 1);
        if selected_bees.is_empty() {
            panic!("Unable to select bee: {:?}", onlookers)
        } else {
            selected_bees[0]
        }
    }
}