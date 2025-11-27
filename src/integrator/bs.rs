use pyo3::prelude::*;

#[pyclass]
pub struct Bs {
    pub internal: bima_rs::integrator::Bs,
}

#[pymethods]
impl Bs {
    #[new]
    fn new(dt: f64, tol: f64, n_try: usize) -> Self {
        Self {
            internal: bima_rs::integrator::Bs::new(dt, tol, n_try),
        }
    }
}