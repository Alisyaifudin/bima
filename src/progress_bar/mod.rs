use pyo3::prelude::*;
use std::time::Instant;
use std::u64;

use crate::progress_bar::py_stdout::PyStdout;
pub mod py_stdout;

pub trait Wrt {
    fn write(&self, str: &str) -> PyResult<()>;
}

pub struct ProgressBar<W: Wrt> {
    length: usize,
    last_time: Instant,
    last_iteration: usize,
    writer: W,
}

fn print_bar_start<'py, W: Wrt>(wrt: &W, length: usize) -> PyResult<()> {
    let progress_str = format!("\r[{}] 0% (0) [??? it/s]", " ".repeat(length));
    wrt.write(&progress_str)?;
    Ok(())
}
fn print_bar<'py, W: Wrt>(
    wrt: &W,
    percentage: f32,
    length: usize,
    total_iteration: usize,
    speed: u64,
) -> PyResult<()> {
    let current = (percentage * length as f32) as usize;
    let progress_str = format!(
        "\r[{}{}] {:.2}% ({total_iteration}) [{speed} it/s]",
        "#".repeat(current),
        " ".repeat(length - current),
        percentage.min(1.) * 100.0,
    );
    wrt.write(&progress_str)?;
    Ok(())
}

fn calc_speed(num_iteration: usize, delta_ms: u128) -> u64 {
    if delta_ms == 0 {
        u64::MAX
    } else {
        1000 * (num_iteration as u64) / delta_ms as u64
    }
}

impl<'py> ProgressBar<PyStdout<'py>> {
    pub fn from_py(py: &Python<'py>, length: usize) -> PyResult<Self> {
        let writer = PyStdout::new(&py)?;
        print_bar_start(&writer, length)?;
        Ok(ProgressBar {
            length,
            writer,
            last_iteration: 0,
            last_time: Instant::now(),
        })
    }
}

impl<W: Wrt> ProgressBar<W> {
    pub fn update(&mut self, iteration: usize, percentage: f32) -> Result<(), PyErr> {
        if iteration < self.last_iteration {
            return Ok(());
        }
        let now = Instant::now();
        let delta_ms = now.duration_since(self.last_time).as_millis();
        let delta = iteration - self.last_iteration;
        let speed = calc_speed(delta, delta_ms);
        self.last_iteration = iteration;
        self.last_time = now;
        print_bar(&self.writer, percentage, self.length, iteration, speed)?;
        if percentage == 1. {
            self.writer.write("\n")?;
        }
        Ok(())
    }
}
