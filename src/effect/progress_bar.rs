use pyo3::prelude::*;
use std::time::SystemTime;
use std::u64;

pub trait Wrt {
    fn write(&self, str: &str) -> PyResult<()>;
}

pub struct ProgressBar<W: Wrt> {
    total: usize,
    t_stop: f64,
    t: SystemTime,
    it: usize,
    current: usize,
    writer: W,
}

fn print_bar_start<'py, W: Wrt>(wrt: &W, total: usize) -> PyResult<()> {
    let progress_str = format!("\r[{}] 0% [??? it/s]", " ".repeat(total));
    wrt.write(&progress_str)?;
    Ok(())
}
fn print_bar<'py, W: Wrt>(
    wrt: &W,
    percent: f64,
    total: usize,
    current: usize,
    it: usize,
    delta_ms: u128,
) -> PyResult<()> {
    let speed = calc_speed(it, delta_ms);
    let progress_str = format!(
        "\r[{}{}] {:.2}% [{speed} it/s]",
        "#".repeat(current),
        " ".repeat(total - current),
        percent.min(1.) * 100.0
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
    pub fn new(writer: W, total: usize, t_stop: f64) -> PyResult<Self> {
        print_bar_start(&writer, total)?;
        Ok(ProgressBar {
            total,
            current: 0,
            it: 0,
            t: SystemTime::now(),
            t_stop,
            writer,
        })
    }
}

pub enum UpdateState {
    Done,
    Print(usize),
    Nothing,
}

impl<W: Wrt> ProgressBar<W> {
    pub fn update(&mut self, t: f64) -> Result<UpdateState, PyErr> {
        self.it += 1;
        let now = SystemTime::now();
        let delta_ms = now
            .duration_since(self.t)
            .map(|d| d.as_millis())
            .unwrap_or(0);
        let percent = t / self.t_stop;
        self.current = (percent * self.total as f64) as usize;
        if delta_ms > 100 {
            print_bar(
                &self.writer,
                percent,
                self.total,
                self.current,
                self.it,
                delta_ms,
            )
            .unwrap();
            self.it = 0;
            self.t = now;
            return Ok(UpdateState::Print(self.it));
        }

        if self.current >= self.total {
            print_bar(
                &self.writer,
                percent,
                self.total,
                self.current,
                self.it,
                delta_ms,
            )?;
            println!();
            return Ok(UpdateState::Done);
        }
        Ok(UpdateState::Nothing)
    }
}
