use crate::core::close_encounter::CloseEncounter;
use crate::core::force::ForceMethod;
use crate::core::solve::SolveMethod;
use crate::core::timestep::TimestepMethod;
use crate::core::vec3::Vec3;
use pyo3::{exceptions::PyValueError, prelude::*};

pub fn some_acc(a: Vec3, saved: bool) -> Option<Vec3> {
    if saved { Some(a) } else { None }
}

pub struct ForceMethodErr;

impl From<ForceMethodErr> for PyErr {
    fn from(_: ForceMethodErr) -> Self {
        PyValueError::new_err("Invalid input")
    }
}
pub fn get_force(force_method: u8) -> Result<ForceMethod, ForceMethodErr> {
    match force_method {
        0 => Ok(ForceMethod::Direct),
        1 => Ok(ForceMethod::new_octree()),
        _ => Err(ForceMethodErr),
    }
}
pub struct SolveMethodErr;
impl From<SolveMethodErr> for PyErr {
    fn from(_: SolveMethodErr) -> Self {
        PyValueError::new_err("Invalid input")
    }
}
pub fn get_solve(solve_method: u8) -> Result<SolveMethod, SolveMethodErr> {
    match solve_method {
        0 => Ok(SolveMethod::Euler),
        1 => Ok(SolveMethod::RK4),
        _ => Err(SolveMethodErr),
    }
}
pub enum TimestepMethodErr {
    NoDelta,
    Invalid,
}
impl From<TimestepMethodErr> for PyErr {
    fn from(v: TimestepMethodErr) -> Self {
        match v {
            TimestepMethodErr::Invalid => PyValueError::new_err("Invalid input"),
            TimestepMethodErr::NoDelta => PyValueError::new_err("No delta value"),
        }
    }
}
pub fn get_timestep(
    timestep_method: u8,
    delta_t: Option<f64>,
) -> Result<TimestepMethod, TimestepMethodErr> {
    match timestep_method {
        0 => {
            let delta_t = delta_t.ok_or(TimestepMethodErr::NoDelta)?;
            Ok(TimestepMethod::Constant(delta_t))
        }
        _ => Err(TimestepMethodErr::Invalid),
    }
}

pub enum CloseEncounterErr {
    NoPar,
    Invalid,
}
impl From<CloseEncounterErr> for PyErr {
    fn from(v: CloseEncounterErr) -> Self {
        match v {
            CloseEncounterErr::Invalid => PyValueError::new_err("Invalid input"),
            CloseEncounterErr::NoPar => PyValueError::new_err("No delta value"),
        }
    }
}
pub fn get_close(
    close_encounter: u8,
    par: Option<f64>,
) -> Result<CloseEncounter, CloseEncounterErr> {
    match close_encounter {
        0 => {
            let par = par.ok_or(CloseEncounterErr::NoPar)?;
            Ok(CloseEncounter::Truncated(par))
        }
        1 => {
            let par = par.ok_or(CloseEncounterErr::NoPar)?;
            Ok(CloseEncounter::Soften(par))
        }
        2 => Ok(CloseEncounter::Regularized),
        _ => Err(CloseEncounterErr::Invalid),
    }
}
