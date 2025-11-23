use bima_rs::effect::{Effect, PayloadRef};
use pyo3::{PyErr, PyResult, Python};

use crate::energy::{progress_bar::ProgressBar, py_stdout::PyStdout};

pub struct Updater<'py> {
    py: Python<'py>,
    progress_bar: ProgressBar<PyStdout<'py>>,
}

impl<'py> Updater<'py> {
    pub fn new(py: Python<'py>) -> PyResult<Self> {
        let writer = PyStdout::new(&py)?;
        let progress_bar = ProgressBar::new(writer, 50)?;
        Ok(Updater { progress_bar, py })
    }
}

impl<'py> Effect<PyErr, usize, usize> for Updater<'py> {
    fn update(&mut self, step: usize, payload: PayloadRef<usize>) -> Result<(), PyErr> {
        self.py.check_signals()?;
        self.progress_bar.update(step, *payload.as_ref())
    }
}
