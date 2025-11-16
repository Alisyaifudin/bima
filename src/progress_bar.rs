use bima_rs::effect::Effect;
use pyo3::prelude::*;
use std::time::SystemTime;
use std::u64;
pub struct ProgressBar<'py> {
    total: usize,
    t_stop: f64,
    t: SystemTime,
    it: u64,
    current: usize,
    stdout: Bound<'py, PyAny>,
}

fn print_bar_start<'py>(stdout: &Bound<'py, PyAny>, total: usize) -> PyResult<()> {
    let progress_str = format!("\r[{}] 0% [??? it/s]", " ".repeat(total));
    stdout.call_method1("write", (progress_str,))?;
    stdout.call_method0("flush")?;
    Ok(())
}
fn print_bar<'py>(
    stdout: &Bound<'py, PyAny>,
    percent: f64,
    total: usize,
    current: usize,
    it: u64,
    delta_ms: u128,
) -> PyResult<()> {
    let speed = calc_speed(it, delta_ms);
    let progress_str = format!(
        "\r[{}{}] {:.2}% [{speed} it/s]",
        "#".repeat(current),
        " ".repeat(total - current),
        percent.min(1.) * 100.0
    );
    stdout.call_method1("write", (progress_str,))?;
    stdout.call_method0("flush")?;
    Ok(())
}

fn calc_speed(it: u64, delta_ms: u128) -> u64 {
    if delta_ms == 0 {
        u64::MAX
    } else {
        1000 * it / delta_ms as u64
    }
}

impl<'py> ProgressBar<'py> {
    pub fn new(py: Python<'py>, total: usize, t_stop: f64) -> PyResult<Self> {
        let sys = PyModule::import(py, "sys")?;
        let stdout = sys.getattr("stdout")?;
        print_bar_start(&stdout, total)?;
        Ok(ProgressBar {
            total,
            current: 0,
            it: 0,
            t: SystemTime::now(),
            t_stop,
            stdout,
        })
    }
}

impl<'py> Effect for ProgressBar<'py> {
    fn update(&mut self, t: f64) {
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
                &self.stdout,
                percent,
                self.total,
                self.current,
                self.it,
                delta_ms,
            )
            .unwrap();
            self.it = 0;
            self.t = now;
        }

        if self.current >= self.total {
            print_bar(
                &self.stdout,
                percent,
                self.total,
                self.current,
                self.it,
                delta_ms,
            )
            .unwrap();
            println!();
        }
    }
}
