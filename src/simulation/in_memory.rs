use crate::progress_bar::ProgressBar;
use crate::progress_bar::py_stdout::PyStdout;
use crate::simulation::Simulation;
use crate::simulation::create_system;
use bima_rs::record::Record;
use bima_rs::record::line::Line;
use bima_rs::record::utils::some_acc;
use pyo3::{PyResult, Python};
use std::time::Instant;

pub fn call<'py>(
    simulation: &Simulation,
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
    let save_acc = save_acc.unwrap_or(false);
    let masses = &simulation.bodies.iter().map(|b| b.m).collect();
    let mut record = Record::empty(masses, save_acc);
    let writer = PyStdout::new(&py)?;
    let mut progress_bar = ProgressBar::new(writer, 50)?;
    let system = create_system(
        &simulation.bodies,
        force_method,
        integrator,
        timestep_method,
        close_encounter,
        delta_t,
        ce_par,
    )?;
    let (rx, handle) = system.integrate(t_stop);
    let mut latest_time = Instant::now();
    let mut iteration = 1;
    for data in rx {
        let (t, percentage, bodies) = (data.t, data.percentage, data.bodies);
        let now = Instant::now();
        if now.duration_since(latest_time).as_millis() >= 100 {
            latest_time = now;
            progress_bar.update(iteration, percentage)?;
        }
        if let Some(bodies) = bodies {
            for (i, body) in bodies.into_iter().enumerate() {
                let a = some_acc(body.a, save_acc);
                let line = Line::new(t, body.r, body.v, a);
                record.add(i, line);
            }
        }
        iteration += 1;
    }
    progress_bar.update(iteration, 1.)?;
    let _ = handle.join().expect("Failed to join thread");
    Ok(record.to_vec(&simulation.cm))
}
