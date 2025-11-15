use pyo3::{exceptions::PyValueError, prelude::*};

use crate::core::vec3::Vec3;

#[pyclass]
#[derive(Debug)]
pub struct Initial {
    pub m: f64,
    pub r: Vec3,
    pub v: Vec3,
}

#[pymethods]
impl Initial {
    fn __repr__(&self) -> String {
        format!(
            "Initial(m={:.9}, r={}, v={})",
            self.m,
            self.r.to_str(),
            self.v.to_str(),
        )
    }
    fn __str__(&self) -> String {
        self.__repr__()
    }
}
#[pyfunction]
pub fn set_initial(
    m: Vec<f64>,
    x: Vec<f64>,
    y: Vec<f64>,
    z: Vec<f64>,
    vx: Vec<f64>,
    vy: Vec<f64>,
    vz: Vec<f64>,
) -> PyResult<Vec<Initial>> {
    let n = m.len();
    if n != x.len()
        || n != y.len()
        || n != z.len()
        || n != vx.len()
        || n != vy.len()
        || n != vz.len()
    {
        return Err(PyValueError::new_err("Dimension not same"));
    }
    Ok((0..n)
        .map(|i| Initial {
            m: m[i],
            r: Vec3::new(x[i], y[i], z[i]),
            v: Vec3::new(vx[i], vy[i], vz[i]),
        })
        .collect())
}
