use pyo3::prelude::*;

#[pyclass]
pub struct Rk4 {
    pub internal: bima_rs::integrator::Rk4,
}

#[pymethods]
impl Rk4 {
    #[new]
    fn new(dt: f64) -> Self {
        Self {
            internal: bima_rs::integrator::Rk4::new(dt),
        }
    }
}
