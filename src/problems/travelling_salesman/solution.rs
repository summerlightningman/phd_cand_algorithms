use std::fmt::{Debug, Formatter};
use super::types::City;

pub struct Solution {
    pub path: Vec<City>,
    pub distance: f64,
    pub time: Option<usize>,
    pub fitness: f32,
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut binding = f.debug_struct("TSSolution");
        let mut ds = binding
            .field("path", &self.path)
            .field("distance", &self.distance);

        if let Some(time) = self.time {
            ds = ds.field("time", &time);
        }

        ds.finish()
    }
}