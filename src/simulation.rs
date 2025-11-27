use crate::force::Direct;
use crate::integrator::{Bs, Euler, LeapFrog, Rk4};
use crate::progress_bar::ProgressBar;
use crate::utils::get_body;
use bima_rs::force::Force;
use bima_rs::integrator::Integrator;
use bima_rs::record::Record;
use paste::paste;
use pyo3::prelude::*;
use std::time::Duration;
use std::time::Instant;

fn run<'py, F: Force, I: Integrator<F>>(
    py: Python<'py>,
    force: F,
    integrator: I,
    t_stop: f64,
    save_acc: bool,
    progress: bool,
) -> PyResult<Vec<Vec<Vec<f64>>>> {
    let masses = &force.bodies().iter().map(|b| b.m).collect();
    let mut record = Record::empty(masses, save_acc);
    let cm = force.cm().clone();
    let mut list_lines = Vec::new();
    // inline the progress bar, instead of trait shenanigan
    if progress {
        let mut iteration = 0;
        let mut last_time = Instant::now();
        let mut bar = ProgressBar::from_py(&py, 50)?;
        let delay = Duration::from_millis(100);
        for lines in integrator.iter(t_stop, save_acc, force) {
            let now = Instant::now();
            if now.duration_since(last_time) >= delay {
                last_time = now;
                let percentage = lines[0].1.t as f32 / t_stop as f32;
                bar.update(iteration, percentage)?;
            }
            list_lines.push(lines);
            iteration += 1;
        }
        bar.update(iteration, 1.)?;
    } else {
        for lines in integrator.iter(t_stop, save_acc, force) {
            list_lines.push(lines);
        }
    }
    for lines in list_lines {
        for (id, line) in lines.into_iter() {
            record.add(id, line);
        }
    }
    Ok(record.to_vec(&cm))
}
fn run_chunk<'py, F: Force, I: Integrator<F>>(
    py: Python<'py>,
    force: F,
    integrator: I,
    t_stop: f64,
    chunk: usize,
    save_acc: bool,
    progress: bool,
) -> PyResult<Vec<Vec<Vec<f64>>>> {
    let masses = &force.bodies().iter().map(|b| b.m).collect();
    let mut record = Record::empty(masses, save_acc);
    let cm = force.cm().clone();
    let mut list_lines = Vec::new();
    // inline the progress bar, instead of trait shenanigan
    if progress {
        let mut iteration = 0;
        let mut last_time = Instant::now();
        let mut bar = ProgressBar::from_py(&py, 50)?;
        let delay = Duration::from_millis(100);
        for lines in integrator.iter(t_stop, save_acc, force) {
            let now = Instant::now();
            if now.duration_since(last_time) >= delay {
                last_time = now;
                let percentage = lines[0].1.t as f32 / t_stop as f32;
                bar.update(iteration, percentage)?;
            }
            list_lines.push(lines);
            iteration += 1;
            if iteration >= chunk {
                break;
            }
        }
        bar.update(iteration, 1.)?;
    } else {
        let limit = chunk - 1;
        for (i, lines) in integrator.iter(t_stop, save_acc, force).enumerate() {
            list_lines.push(lines);
            if i >= limit {
                break;
            }
        }
    }
    for lines in list_lines {
        for (id, line) in lines.into_iter() {
            record.add(id, line);
        }
    }
    Ok(record.to_vec(&cm))
}
macro_rules! create_simulation {
    ($force: ident, $integrator: ident) => {
        paste! {
        #[pyclass]
        #[allow(non_camel_case_types)]
        pub struct [<$force _ $integrator>] {
            integrator: bima_rs::integrator::$integrator,
            force: bima_rs::force::$force,
        }

        #[pymethods]
        impl [<$force _ $integrator>] {
            #[new]
            fn new(integrator: Bound<'_, $integrator>, force: Bound<'_, $force>) -> Self {
                let integrator = integrator.borrow().internal.clone();
                let force = force.borrow().internal.clone();
                Self { integrator, force }
            }

            #[pyo3(signature = (bodies, n_active, t_stop, save_acc, progress))]
            fn run<'py>(
                &self,
                py: Python<'py>,
                bodies: Vec<[f64; 7]>,
                n_active: usize,
                t_stop: f64,
                save_acc: bool,
                progress: bool,
            ) -> PyResult<Vec<Vec<Vec<f64>>>> {
                let bodies = get_body(bodies);
                let force = self.force.with_bodies(bodies, n_active);
                run(
                    py,
                    force,
                    self.integrator.clone(),
                    t_stop,
                    save_acc,
                    progress,
                )
            }
            #[pyo3(signature = (bodies, n_active, t_stop, chunk, save_acc, progress))]
            fn run_chunk<'py>(
                &self,
                py: Python<'py>,
                bodies: Vec<[f64; 7]>,
                n_active: usize,
                t_stop: f64,
                chunk: usize,
                save_acc: bool,
                progress: bool,
            ) -> PyResult<Vec<Vec<Vec<f64>>>> {
                let bodies = get_body(bodies);
                let force = self.force.with_bodies(bodies, n_active);
                run_chunk(
                    py,
                    force,
                    self.integrator.clone(),
                    t_stop,
                    chunk,
                    save_acc,
                    progress,
                )
            }
        }
        }
    };
}
create_simulation!(Direct, Euler);
create_simulation!(Direct, Rk4);
create_simulation!(Direct, Bs);
create_simulation!(Direct, LeapFrog);
