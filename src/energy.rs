use crate::progress_bar::ProgressBar;
use bima_rs::record::line::Line;
use bima_rs::record::trajectory::Trajectory;
use bima_rs::vec3::Vec3;
use bima_rs::{energy::Energy, record::Record};
use numpy::{IntoPyArray, PyArray1};
use pyo3::{exceptions::PyValueError, prelude::*};
use std::time::{Duration, Instant};

#[pyfunction]
pub fn calc_energy<'py>(
    py: Python<'py>,
    objects: Vec<Vec<[f64; 7]>>,
    masses: Vec<f64>,
    n_active: usize,
    progress: bool,
) -> PyResult<[Py<PyArray1<f64>>; 2]> {
    if objects.len() != masses.len() {
        return Err(PyValueError::new_err(
            "masses and objects must have the same length",
        ));
    }
    let record = create_record(objects, masses);
    // eprintln!("traj {:?}", record.objects[0]);
    let energy =
        Energy::new(record, n_active).map_err(|_| PyValueError::new_err("Empty record"))?;
    let mut times = Vec::new();
    let mut energies = Vec::new();
    let length = energy.length;
    if progress {
        let mut last_time = Instant::now();
        let mut bar = ProgressBar::from_py(&py, 50)?;
        let delay = Duration::from_millis(100);
        for (iteration, data) in energy.into_iter().enumerate() {
            let now = Instant::now();
            if now.duration_since(last_time) >= delay {
                last_time = now;
                let percentage = iteration as f32 / length as f32;
                bar.update(iteration, percentage)?;
            }
            times.push(data.time);
            energies.push(data.energy);
        }
        bar.update(length, 1.)?;
    } else {
        for data in energy.into_iter() {
            times.push(data.time);
            energies.push(data.energy);
        }
    }
    Ok([
        times.into_pyarray(py).into(),
        energies.into_pyarray(py).into(),
    ])
}

fn create_record(objects: Vec<Vec<[f64; 7]>>, masses: Vec<f64>) -> Record {
    let n = objects.len();
    let mut trajectories = Vec::with_capacity(n);
    for (object, mass) in objects.into_iter().zip(masses.into_iter()) {
        let lines = object
            .into_iter()
            .map(|path| {
                Line::new(
                    path[0],
                    Vec3::new(path[1], path[2], path[3]),
                    Vec3::new(path[4], path[5], path[6]),
                    None,
                )
            })
            .collect();
        let traj = Trajectory::from_lines(lines, mass);
        trajectories.push(traj);
    }
    let record = Record::from_trajectories(trajectories, false);
    record
}
