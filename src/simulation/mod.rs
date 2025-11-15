mod utils;
use crate::core::cm::CM;
use crate::core::record::{Line, Record};
use crate::core::system::{Body, System};
use crate::core::update::update_loop;
use crate::initial::Initial;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

#[pyclass]
pub struct Simulation {
    record: Record,
    cm: CM,
    bodies: Vec<Body>,
    save_acc: bool,
}

#[pymethods]
impl Simulation {
    #[new]
    #[pyo3(signature = (initial, save_acc=None))]
    fn new(initial: Vec<Bound<'_, Initial>>, save_acc: Option<bool>) -> PyResult<Self> {
        let save_acc = save_acc.unwrap_or(false);
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
                body.r -= cm.r;
                body.v -= cm.v;
                return body;
            })
            .collect();
        let n = relative_bodies.len();
        let mut lines = Vec::with_capacity(n);
        for b in relative_bodies.clone() {
            let line = Line::new(0.0, b.r, b.v, utils::some_acc(b.a, save_acc));
            lines.push(vec![line]);
        }
        let record = Record(lines);
        Ok(Simulation {
            record,
            cm,
            bodies: relative_bodies,
            save_acc,
        })
    }
    #[getter]
    fn record(&self) -> Vec<Vec<Vec<f64>>> {
        self.record
            .0
            .clone()
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|line| {
                        if let Some(a) = line.a {
                            vec![
                                line.t,
                                line.r.x() + self.cm.r.x(),
                                line.r.y() + self.cm.r.y(),
                                line.r.z() + self.cm.r.z(),
                                line.v.x() + self.cm.r.x(),
                                line.v.y() + self.cm.r.y(),
                                line.v.z() + self.cm.r.z(),
                                a.x(),
                                a.y(),
                                a.z(),
                            ]
                        } else {
                            vec![
                                line.t,
                                line.r.x() + self.cm.r.x(),
                                line.r.y() + self.cm.r.y(),
                                line.r.z() + self.cm.r.z(),
                                line.v.x() + self.cm.r.x(),
                                line.v.y() + self.cm.r.y(),
                                line.v.z() + self.cm.r.z(),
                            ]
                        }
                    })
                    .collect()
            })
            .collect()
    }

    #[pyo3(signature = (force_method, solve_method, timestep_method, close_encounter, t_stop, delta_t=None, ce_par=None))]
    fn run(
        &mut self,
        py: Python,
        force_method: u8,
        solve_method: u8,
        timestep_method: u8,
        close_encounter: u8,
        t_stop: f64,
        delta_t: Option<f64>,
        ce_par: Option<f64>,
    ) -> PyResult<()> {
        let force = utils::get_force(force_method)?;
        let solve = utils::get_solve(solve_method)?;
        let timestep = utils::get_timestep(timestep_method, delta_t)?;
        let close = utils::get_close(close_encounter, ce_par)?;
        let mut system = System {
            t: 0.0,
            bodies: self.bodies.clone(),
            force_method: force,
            solve_method: solve,
            timestep_method: timestep,
            close_encounter: close,
            save_acc: self.save_acc,
        };
        update_loop(py, &mut system, t_stop, &mut self.record)?;
        Ok(())
    }
}
