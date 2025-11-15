use crate::core::record::Record;
use crate::core::system::System;
use crate::core::timestep::{self, TimestepMethod};
use crate::progress_bar::ProgressBar;
use pyo3::prelude::*;

pub fn update_loop(
    py: Python,
    system: &mut System,
    t_stop: f64,
    record: &mut Record,
) -> PyResult<()> {
    let mut progress = ProgressBar::new(1000, t_stop);
    match system.timestep_method {
        TimestepMethod::Constant(delta_t) => {
            if delta_t <= 0.0 {
                return Ok(());
            }
            while system.t < t_stop {
                py.check_signals()?;
                system.calc_forces();
                timestep::constant_step(system, delta_t, record);
                system.t += delta_t;
                progress.update(system.t);
            }
        }
    };
    Ok(())
}
