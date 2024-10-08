use rand::rngs::ThreadRng;
use crate::algorithms::bee_colony::types::ResearchFunction;
use crate::algorithms::helpers;


pub fn swap_indexes<T: Clone>(offset: Option<usize>) -> ResearchFunction<T> {
    ResearchFunction(Box::new(move |value, rng: &mut ThreadRng| {
        let (left, right) = helpers::generate_two_points(offset, value.len(), rng);
        let mut source = value.to_vec();
        source.swap(left, right);
        source
    }))
}

pub fn reverse_elements<T: Clone>(offset: Option<usize>) -> ResearchFunction<T> {
    ResearchFunction(Box::new(move |value, rng: &mut ThreadRng| {
        let (left, right) = helpers::generate_two_points(offset, value.len(), rng);
        let mut value_new = value.to_vec();

        value_new[left..right].reverse();
        value_new
    }))
}