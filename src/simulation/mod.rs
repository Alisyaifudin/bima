mod utils;
use crate::effect::disk::InDisk;
use crate::effect::memory::InMemory;
use crate::initial::Initial;
use bima_rs::cm::CM;
use bima_rs::record::Record;
use bima_rs::system::{Body, System};
use bima_rs::update::update_loop;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct Simulation {
    cm: CM,
    bodies: Vec<Body>,
}

#[pymethods]
impl Simulation {
    #[new]
    #[pyo3(signature = (initial))]
    fn new(initial: Vec<Bound<'_, Initial>>) -> PyResult<Self> {
        let bodies = initial
            .iter()
            .map(|obj| {
                let initial = obj.borrow();
                Body::new(initial.m, initial.r, initial.v)
            })
            .collect::<Vec<Body>>();
        let cm =
            CM::from_bodies(&bodies).map_err(|_| PyValueError::new_err("Total mass is zero"))?;
        let relative_bodies: Vec<Body> = bodies
            .into_iter()
            .map(|mut body| {
                body.r -= cm.r();
                return body;
            })
            .collect();
        Ok(Simulation {
            cm,
            bodies: relative_bodies,
        })
    }
    #[pyo3(signature = (force_method, solve_method, timestep_method, close_encounter, t_stop, delta_t=None, ce_par=None, save_acc=None))]
    fn run_memory<'py>(
        &mut self,
        py: Python<'py>,
        force_method: u8,
        solve_method: u8,
        timestep_method: u8,
        close_encounter: u8,
        t_stop: f64,
        delta_t: Option<f64>,
        ce_par: Option<f64>,
        save_acc: Option<bool>,
    ) -> PyResult<Vec<Vec<Vec<f64>>>> {
        let save_acc = save_acc.unwrap_or(false);
        let mut record = Record::new(&self.bodies, save_acc);
        let mut system = create_system(
            &self.bodies,
            force_method,
            solve_method,
            timestep_method,
            close_encounter,
            delta_t,
            ce_par,
        )?;
        let mut effect = InMemory::new(py, t_stop, &record)?;
        update_loop(&mut effect, &mut system, t_stop, &mut record)?;
        Ok(effect.record.to_vec(&self.cm))
    }
    #[pyo3(signature = (abs_path, force_method, solve_method, timestep_method, close_encounter, t_stop, delta_t=None, ce_par=None, save_acc=None, replace=None))]
    fn run_disk<'py>(
        &mut self,
        py: Python<'py>,
        abs_path: &str,
        force_method: u8,
        solve_method: u8,
        timestep_method: u8,
        close_encounter: u8,
        t_stop: f64,
        delta_t: Option<f64>,
        ce_par: Option<f64>,
        save_acc: Option<bool>,
        replace: Option<bool>,
    ) -> PyResult<String> {
        let save_acc = save_acc.unwrap_or(false);
        let replace = replace.unwrap_or(false);
        let mut record = Record::new(&self.bodies, save_acc);
        let mut system = create_system(
            &self.bodies,
            force_method,
            solve_method,
            timestep_method,
            close_encounter,
            delta_t,
            ce_par,
        )?;
        let mut effect = InDisk::new(py, t_stop, abs_path, replace, &record, &self.cm)?;
        update_loop(&mut effect, &mut system, t_stop, &mut record)?;
        Ok(effect.get_path())
    }
}

fn create_system(
    bodies: &Vec<Body>,
    force_method: u8,
    solve_method: u8,
    timestep_method: u8,
    close_encounter: u8,
    delta_t: Option<f64>,
    ce_par: Option<f64>,
) -> PyResult<System> {
    let force = utils::get_force(force_method)?;
    let solve = utils::get_solve(solve_method)?;
    let timestep = utils::get_timestep(timestep_method, delta_t)?;
    let close = utils::get_close(close_encounter, ce_par)?;
    Ok(System {
        t: 0.0,
        bodies: bodies.clone(),
        force_method: force,
        solve_method: solve,
        timestep_method: timestep,
        close_encounter: close,
    })
}
