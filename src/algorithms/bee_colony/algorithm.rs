use std::error::Error;
use std::fmt::Debug;
use random_choice::random_choice;
use crate::algorithms::types::{Purpose};
use super::types::{FitnessFuncRaw, ResearchFuncRaw, GenerateFuncRaw};
use super::bee::{Bee};


struct BeeColonyAlgorithm<T> {
    pub actors_count: usize,
    pub iters_count: u64,
    pub solutions_count: usize,
    pub workers_part: f32,
    pub purpose: Purpose,
    pub fitness_func: FitnessFuncRaw<T>,
    pub research_func: ResearchFuncRaw<T>,
    pub generate_func: GenerateFuncRaw<T>,
}

impl<T: Clone + Debug> BeeColonyAlgorithm<T> {
    pub fn run(&self) -> Result<Vec<Bee<T>>, &str> {
        let workers_count = (self.workers_part * self.actors_count as f32).round() as usize;
        let onlookers_count = self.actors_count - workers_count;
        let mut workers: Vec<Bee<T>> = (0..workers_count).map(|_| { self.generate_bee() }).collect();

        for _ in 0..self.iters_count {
            let onlookers: Vec<Bee<T>> = (0..onlookers_count).map(|_| { self.generate_bee() }).collect();

            let probabilities = self.get_source_probabilities(&onlookers);
            for worker in workers.iter_mut() {
                let selected_source: &Bee<T> = self.select_onlooker_by_probabilities(&onlookers, &probabilities);

                let researched_source = (self.research_func)(&worker.source);
                let researched_fitness = (self.fitness_func)(&worker.source);

                if let Purpose::Min = self.purpose {
                    if researched_fitness < selected_source.fitness && researched_fitness < worker.fitness {
                        worker.source = researched_source;
                        worker.fitness = researched_fitness;
                    } else if selected_source.fitness < worker.fitness {
                        worker.source = selected_source.source.clone();
                        worker.fitness = selected_source.fitness;
                    }
                } else {
                    if researched_fitness > selected_source.fitness && researched_fitness > worker.fitness {
                        worker.source = researched_source;
                        worker.fitness = researched_fitness;
                    } else if selected_source.fitness > worker.fitness {
                        worker.source = selected_source.source.clone();
                        worker.fitness = selected_source.fitness;
                    }
                }
            }
        }

        Ok(workers)
    }

    fn generate_bee(&self) -> Bee<T> {
        let source = (self.generate_func)();
        let fitness = (self.fitness_func)(&source);

        Bee {
            source,
            fitness,
        }
    }

    fn get_source_probabilities(&self, onlookers: &Vec<Bee<T>>) -> Vec<f64> {
        let fitness_sum: f64 = onlookers.iter().map(|bee| {bee.fitness}).sum();
        let get_probability: fn(f64, f64) -> f64 = match self.purpose {
            Purpose::Min => |fit: f64, sum: f64| { 1. - fit / sum },
            Purpose::Max => |fit: f64, sum: f64| { fit / sum }
        };
        return onlookers.iter().map(|bee| {
            get_probability(bee.fitness, fitness_sum)
        }).collect();
    }

    fn select_onlooker_by_probabilities<'a>(&self, onlookers: &'a Vec<Bee<T>>, probabilities: &Vec<f64>) -> &'a Bee<T> {
        let selected_bees = random_choice().random_choice_f64(&onlookers, &probabilities, 1);
        if selected_bees.is_empty() {
            panic!("Unable to select bee: {:?}", onlookers)
        } else {
            selected_bees[0]
        }
    }
}