use pyo3::prelude::*;

#[pyclass]
pub struct Direct {
    pub internal: bima_rs::force::Direct,
}

#[pymethods]
impl Direct {
    #[new]
    fn new(s: f64) -> Self {
        Self {
            internal: bima_rs::force::Direct::empty(s),
        }
    }
}
