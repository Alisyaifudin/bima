use pyo3::prelude::*;

#[pyclass]
pub struct LeapFrog {
    pub internal: bima_rs::integrator::LeapFrog,
}

#[pymethods]
impl LeapFrog {
    #[new]
    fn new(dt: f64) -> Self {
        Self {
            internal: bima_rs::integrator::LeapFrog::new(dt),
        }
    }
}