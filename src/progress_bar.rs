use tqdm::{Tqdm, pbar};
pub struct ProgressBar {
    total: usize,
    t_stop: f64,
    current: usize,
    pbar: Tqdm<()>,
}

impl ProgressBar {
    pub fn new(total: usize, t_stop: f64) -> Self {
        ProgressBar {
            total,
            current: 0,
            t_stop,
            pbar: pbar(Some(total)),
        }
    }
    pub fn update(&mut self, t: f64) {
        let progress = (self.total as f64 * (t / self.t_stop)) as usize;
        let progress = progress.min(self.total);

        let delta = progress.saturating_sub(self.current);
        if delta > 0 {
            self.pbar.update(delta).unwrap();
            self.current = progress;
        }
    }
}
