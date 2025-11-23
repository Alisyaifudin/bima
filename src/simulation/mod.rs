mod in_disk;
mod in_memory;
mod store;
mod utils;
use crate::initial::Initial;
use bima_rs::body::Body;
use bima_rs::cm::CM;
use bima_rs::system::System;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashMap;

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
            .enumerate()
            .map(|(i, obj)| {
                let initial = obj.borrow();
                Body::new(i, initial.m, initial.r, initial.v, None)
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
    #[pyo3(signature = (force_method, integrator, timestep_method, close_encounter, t_stop, delta_t=None, ce_par=None, save_acc=None))]
    fn run_memory<'py>(
        &self,
        py: Python<'py>,
        force_method: u8,
        integrator: u8,
        timestep_method: u8,
        close_encounter: u8,
        t_stop: f64,
        delta_t: Option<f64>,
        ce_par: Option<f64>,
        save_acc: Option<bool>,
    ) -> PyResult<Vec<Vec<Vec<f64>>>> {
        in_memory::call(
            &self,
            py,
            force_method,
            integrator,
            timestep_method,
            close_encounter,
            t_stop,
            delta_t,
            ce_par,
            save_acc,
        )
    }
    #[pyo3(signature = (abs_path, force_method, integrator, timestep_method, close_encounter, t_stop, delta_t=None, ce_par=None, save_acc=None, replace=None))]
    fn run_disk<'py>(
        &self,
        py: Python<'py>,
        abs_path: &str,
        force_method: u8,
        integrator: u8,
        timestep_method: u8,
        close_encounter: u8,
        t_stop: f64,
        delta_t: Option<f64>,
        ce_par: Option<f64>,
        save_acc: Option<bool>,
        replace: Option<bool>,
    ) -> PyResult<String> {
        in_disk::call(
            self,
            py,
            abs_path,
            force_method,
            integrator,
            timestep_method,
            close_encounter,
            t_stop,
            delta_t,
            ce_par,
            save_acc,
            replace,
        )
    }
}

fn create_system(
    bodies: &Vec<Body>,
    force_method: u8,
    integrator: u8,
    timestep_method: u8,
    close_encounter: u8,
    delta_t: Option<f64>,
    ce_par: Option<f64>,
) -> PyResult<System> {
    let force_method = utils::get_force(force_method)?;
    let integrator = utils::get_integrator(integrator)?;
    let timestep_method = utils::get_timestep(timestep_method, delta_t)?;
    let close_encounter = utils::get_close(close_encounter, ce_par)?;
    Ok(System {
        t: 0.0,
        bodies: bodies.clone(),
        force_method,
        integrator,
        timestep_method,
        close_encounter,
        cache: HashMap::new(),
    })
}
