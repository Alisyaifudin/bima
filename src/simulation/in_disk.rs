use crate::progress_bar::ProgressBar;
use crate::progress_bar::py_stdout::PyStdout;
use crate::simulation::Simulation;
use crate::simulation::create_system;
use crate::simulation::store::Store;
use bima_rs::record::Record;
use bima_rs::record::line::Line;
use bima_rs::record::utils::some_acc;
use pyo3::PyErr;
use pyo3::exceptions::PyValueError;
use pyo3::{PyResult, Python};
use std::fs;
use std::io;
use std::path::PathBuf;
use std::time::Instant;

enum RootPathErr {
    AlreadyExistAsFile,
    FailedToCreate,
}

impl From<RootPathErr> for PyErr {
    fn from(value: RootPathErr) -> Self {
        match value {
            RootPathErr::AlreadyExistAsFile => PyValueError::new_err("The given path is a file"),
            RootPathErr::FailedToCreate => {
                PyValueError::new_err("Something went wrong, failed to create the dir")
            }
        }
    }
}

fn gen_dir_path(abs_path: &str) -> Result<PathBuf, RootPathErr> {
    let dir_path = PathBuf::from(abs_path);
    match fs::create_dir_all(&dir_path) {
        Ok(_) => return Ok(dir_path),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(RootPathErr::FailedToCreate),
    }
    let metadata = dir_path
        .metadata()
        .expect("Already check above, so must exist");
    if metadata.is_file() {
        return Err(RootPathErr::AlreadyExistAsFile);
    }
    return Ok(dir_path);
}

pub fn call<'py>(
    simulation: &Simulation,
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
    let save_acc = save_acc.unwrap_or(false);
    let replace = replace.unwrap_or(false);
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
    let dir_path = gen_dir_path(abs_path)?;
    let dir_path = fs::canonicalize(dir_path)?;
    let file_path = dir_path.join("res.h5");
    let mut store = Store::new(file_path, record.len(), masses.clone(), replace, save_acc)?;
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
        iteration += 1;
        bodies
            .into_iter()
            .flatten()
            .enumerate()
            .for_each(|(i, body)| {
                let a = some_acc(body.a, save_acc);
                let line = Line::new(t, body.r, body.v, a);
                record.add(i, line);
            });
        if record.len() >= 65536 {
            for obj_id in 0..record.len() {
                let lines = record.take(obj_id);
                store
                    .append(obj_id, lines.path, &simulation.cm)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
            }
        }
    }
    // last one
    progress_bar.update(iteration, 1.0)?;
    for obj_id in 0..record.len() {
        let lines = record.take(obj_id);
        store
            .append(obj_id, lines.path, &simulation.cm)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
    }
    let _ = handle.join().unwrap();
    Ok(store.path.to_string_lossy().into())
}
