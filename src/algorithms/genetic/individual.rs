#[derive(Clone, Debug)]
pub struct Individual<T> {
    pub value: Vec<T>,
    pub fitness: Option<f64>,
}

impl<T> Individual<T> {
    pub fn new(value: Vec<T>, fitness: Option<f64>) -> Self {
        Self {
            value,
            fitness
        }
    }
}
