use crate::effect::progress_bar::{ProgressBar, UpdateState};
use crate::effect::py_stdout::PyStdout;
use crate::effect::store::Store;
use bima_rs::cm::CM;
use bima_rs::effect::Effect;
use bima_rs::record::Record;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct InDisk<'py> {
    py: Python<'py>,
    progress_bar: ProgressBar<PyStdout<'py>>,
    store: Store,
    cm: &'py CM,
}

enum RootPathErr {
    AlreadyExistAsFile,
    FailedToCreate,
}

impl From<RootPathErr> for PyErr {
    fn from(value: RootPathErr) -> Self {
        match value {
            RootPathErr::AlreadyExistAsFile => PyValueError::new_err("The given path is a file"),
            RootPathErr::FailedToCreate => {
                PyValueError::new_err("Something went wrong, failed to create the dir")
            }
        }
    }
}

fn gen_dir_path(abs_path: &str) -> Result<PathBuf, RootPathErr> {
    let dir_path = PathBuf::from(abs_path);
    match fs::create_dir_all(&dir_path) {
        Ok(_) => return Ok(dir_path),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {}
        Err(_) => return Err(RootPathErr::FailedToCreate),
    }
    let metadata = dir_path
        .metadata()
        .expect("Already check above, so must exist");
    if metadata.is_file() {
        return Err(RootPathErr::AlreadyExistAsFile);
    }
    return Ok(dir_path);
}

impl<'py> InDisk<'py> {
    pub fn new(
        py: Python<'py>,
        t_stop: f64,
        abs_path: &str,
        replace: bool,
        record: &Record,
        cm: &'py CM,
    ) -> PyResult<Self> {
        let writer = PyStdout::new(&py)?;
        let progress_bar = ProgressBar::new(writer, 50, t_stop)?;
        let dir_path = gen_dir_path(abs_path)?;
        let dir_path = fs::canonicalize(dir_path)?;
        let file_path = dir_path.join("res.h5");
        let store = Store::new(file_path, record.len(), replace, record.save_acc)?;
        Ok(InDisk {
            progress_bar,
            py,
            cm,
            store,
        })
    }
    fn store(&mut self, record: &mut Record) -> PyResult<()> {
        for obj_id in 0..record.len() {
            let lines = record.take(obj_id);
            self.store
                .append(obj_id, lines, &self.cm)
                .map_err(|e| PyValueError::new_err(e.to_string()))?;
        }
        Ok(())
    }
    pub fn get_path(&self) -> String {
        self.store.path.to_string_lossy().into()
    }
}

impl<'py> Effect<PyErr> for InDisk<'py> {
    fn update(&mut self, t: f64, record: &mut Record) -> Result<(), PyErr> {
        self.py.check_signals()?;
        let res = self.progress_bar.update(t)?;
        let should_store = match res {
            UpdateState::Done => true,
            UpdateState::Nothing => false,
            UpdateState::Print(it) => it > 10,
        };
        if should_store {
            self.store(record)?;
        }
        Ok(())
    }
}
