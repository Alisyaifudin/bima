use pyo3::prelude::*;

use crate::worker::progress_bar::Wrt;


pub struct PyStdout<'py> {
    stdout: Bound<'py, PyAny>,
}

impl<'py> PyStdout<'py> {
    pub fn new(py: &Python<'py>) -> PyResult<Self> {
        let sys = PyModule::import(*py, "sys")?;
        let stdout = sys.getattr("stdout")?;
        Ok(PyStdout { stdout })
    }
}

impl<'py> Wrt for PyStdout<'py> {
    fn write(&self, str: &str) -> PyResult<()> {
        self.stdout.call_method1("write", (str,))?;
        self.stdout.call_method0("flush")?;
        Ok(())
    }
}
