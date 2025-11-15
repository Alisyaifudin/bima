mod core;
mod initial;
mod simulation;
mod progress_bar;
use initial::set_initial;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "_bima")] // Name must match Cargo.toml
fn _bima(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(set_initial, m)?)?;
    m.add_class::<simulation::Simulation>()?;
    m.add_class::<initial::Initial>()?;
    Ok(())
}
