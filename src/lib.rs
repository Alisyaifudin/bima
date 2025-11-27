mod energy;
mod force;
mod integrator;
mod progress_bar;
mod simulation;
mod utils;
use pyo3::prelude::*;

#[pymodule]
#[pyo3(name = "_bima")] // Name must match Cargo.toml
fn _bima(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(energy::calc_energy, m)?)?;
    m.add_class::<force::Direct>()?;
    m.add_class::<integrator::Euler>()?;
    m.add_class::<integrator::Rk4>()?;
    m.add_class::<integrator::Bs>()?;
    m.add_class::<integrator::LeapFrog>()?;
    m.add_class::<simulation::Direct_Euler>()?;
    m.add_class::<simulation::Direct_Rk4>()?;
    m.add_class::<simulation::Direct_Bs>()?;
    m.add_class::<simulation::Direct_LeapFrog>()?;
    Ok(())
}
