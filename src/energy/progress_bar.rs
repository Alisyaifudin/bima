use pyo3::prelude::*;
use std::time::SystemTime;
use std::u64;

pub trait Wrt {
    fn write(&self, str: &str) -> PyResult<()>;
}

pub struct ProgressBar<W: Wrt> {
    length: usize,
    time: SystemTime,
    temp_iteration: usize,
    writer: W,
}

fn print_bar_start<'py, W: Wrt>(wrt: &W, length: usize) -> PyResult<()> {
    let progress_str = format!("\r[{}] 0% (0) [??? it/s]", " ".repeat(length));
    wrt.write(&progress_str)?;
    Ok(())
}
fn print_bar<'py, W: Wrt>(
    wrt: &W,
    percent: f64,
    length: usize,
    current: usize,
    it: usize,
    num_of_iteration: usize,
    delta_ms: u128,
) -> PyResult<()> {
    let speed = calc_speed(it, delta_ms);
    let progress_str = format!(
        "\r[{}{}] {:.2}% ({num_of_iteration}) [{speed} it/s]",
        "#".repeat(current),
        " ".repeat(length - current),
        percent.min(1.) * 100.0,
    );
    wrt.write(&progress_str)?;
    Ok(())
}

fn calc_speed(it: usize, delta_ms: u128) -> u64 {
    if delta_ms == 0 {
        u64::MAX
    } else {
        1000 * (it as u64) / delta_ms as u64
    }
}

impl<W: Wrt> ProgressBar<W> {
    pub fn new(writer: W, length: usize) -> PyResult<Self> {
        print_bar_start(&writer, length)?;
        Ok(ProgressBar {
            length,
            temp_iteration: 0,
            time: SystemTime::now(),
            writer,
        })
    }
}

impl<W: Wrt> ProgressBar<W> {
    pub fn update(&mut self, step: usize, total_step: usize) -> Result<(), PyErr> {
        self.temp_iteration += 1;
        let now = SystemTime::now();
        let delta_ms = now
            .duration_since(self.time)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let percent = step as f64 / total_step as f64;
        let current_progress = (percent * self.length as f64) as usize;
        if delta_ms > 100 {
            print_bar(
                &self.writer,
                percent,
                self.length,
                current_progress,
                self.temp_iteration,
                step,
                delta_ms,
            )
            .unwrap();
            self.temp_iteration = 0;
            self.time = now;
            return Ok(());
        }

        if step >= total_step {
            print_bar(
                &self.writer,
                percent,
                self.length,
                current_progress,
                self.temp_iteration,
                total_step + 1,
                delta_ms,
            )?;
            println!();
            return Ok(());
        }
        Ok(())
    }
}
