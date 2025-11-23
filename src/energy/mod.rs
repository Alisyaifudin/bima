use bima_rs::{
    energy,
    record::{Record, line::Line, trajectory::Trajectory},
    vec3::Vec3,
};
use numpy::{IntoPyArray, PyArray1};
use pyo3::{exceptions::PyValueError, prelude::*};

use crate::energy::updater::Updater;
mod progress_bar;
mod py_stdout;
mod updater;

#[pyfunction]
pub fn calc_energy<'py>(
    py: Python<'py>,
    objects: Vec<Vec<[f64; 7]>>,
    masses: Vec<f64>,
) -> PyResult<[Py<PyArray1<f64>>; 2]> {
    if objects.is_empty() {
        return Err(PyValueError::new_err("objects cannot be empty"));
    }
    let n = objects.len();
    if n != masses.len() {
        return Err(PyValueError::new_err(
            "masses and objects must have the same length",
        ));
    }
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
    let mut effect = Updater::new(py)?;
    let energies = energy::calc_energy(&record, &mut effect)
        .map_err(|_| PyValueError::new_err("Empty objects"))?;
    let (times, energy_values): (Vec<f64>, Vec<f64>) = energies.into_iter().unzip();
    Ok([
        times.into_pyarray(py).into(),
        energy_values.into_pyarray(py).into(),
    ])
}

// if path.len() != 7 {
//     return Err(PyValueError::new_err(
//         "Malformed data. Should have only 7 columns",
//     ));
// }
