use crate::worker::progress_bar::{ProgressBar, UpdateState};
use crate::worker::py_stdout::PyStdout;
use bima_rs::effect::{Effect, PayloadRef};
use bima_rs::record::Record;
use pyo3::prelude::*;

pub struct InMemory<'py> {
    py: Python<'py>,
    progress_bar: ProgressBar<PyStdout<'py>>,
    pub record: Record,
}

impl<'py> InMemory<'py> {
    pub fn new(py: Python<'py>, t_stop: f64, masses: &Vec<f64>, save_acc: bool) -> PyResult<Self> {
        let writer = PyStdout::new(&py)?;
        let progress_bar = ProgressBar::new(writer, 50, t_stop)?;
        Ok(InMemory {
            progress_bar,
            py,
            record: Record::empty(masses, save_acc),
        })
    }
    pub fn drain(&mut self, other: &mut Record) {
        for (i, r) in other.objects.iter_mut().enumerate() {
            let take = std::mem::take(r);
            self.record.add_many(i, take.path);
        }
    }
}

impl<'py> Effect<PyErr, f64, Record> for InMemory<'py> {
    fn update(&mut self, t: f64, mut payload: PayloadRef<Record>) -> Result<(), PyErr> {
        self.py.check_signals()?;
        let res = self.progress_bar.update(t)?;
        let should_drain = match res {
            UpdateState::Done => true,
            UpdateState::Nothing => false,
            UpdateState::Print(it) => it > 1000,
        };
        if should_drain {
            if let Some(record) = payload.as_mut() {
                self.drain(record);
            }
        }
        Ok(())
    }
}
