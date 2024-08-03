use std::fmt::{Debug, Formatter};
use crate::problems::travelling_salesman::types::City;

pub struct Solution {
    pub path: Vec<City>,
    pub distance: f64,
    pub time: Option<usize>,
}

impl Debug for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("TSSolution")
            .field("path", &self.path)
            .field("distance", &self.distance);

        if let Some(time) = self.time {
            ds = ds.field("time", &time);
        }

        ds.finish()
    }
}