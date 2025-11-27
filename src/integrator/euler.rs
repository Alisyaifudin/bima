use pyo3::prelude::*;

#[pyclass]
pub struct Euler {
    pub internal: bima_rs::integrator::Euler,
}

#[pymethods]
impl Euler {
    #[new]
    fn new(dt: f64) -> Self {
        Self {
            internal: bima_rs::integrator::Euler::new(dt),
        }
    }
}
