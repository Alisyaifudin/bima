use pyo3::prelude::*;
use std::time::Instant;
use std::u64;
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
    percentage: f64,
    length: usize,
    total_iteration: usize,
    speed: u64,
) -> PyResult<()> {
    let current = (percentage * length as f64) as usize;
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

impl<W: Wrt> ProgressBar<W> {
    pub fn new(writer: W, length: usize) -> PyResult<Self> {
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
    pub fn update(&mut self, iteration: usize, percentage: f64) -> Result<(), PyErr> {
        let now = Instant::now();
        let delta_ms = now.duration_since(self.last_time).as_millis();
        let speed = calc_speed(iteration - self.last_iteration, delta_ms);
        self.last_iteration = iteration;
        self.last_time = now;
        print_bar(&self.writer, percentage, self.length, iteration, speed)?;
        if percentage == 1. {
            self.writer.write("\n")?;
        }
        Ok(())
    }
}
