use std::time::Duration;

use crate::global::performance;

pub struct TickScheduler {
    last_tick: f64,
    interval: f64,
}

fn current_time() -> f64 {
    performance().now()
}

impl TickScheduler {
    #[must_use]
    pub fn new(interval: Duration) -> Self {
        let last_tick = current_time();
        let interval = interval.as_secs_f64() * 1000.;

        Self {
            last_tick,
            interval,
        }
    }

    pub fn tick_count(&mut self) -> u32 {
        let interval = current_time() - self.last_tick;

        let count = (interval / self.interval) as u32;

        self.last_tick += count as f64 * self.interval;

        count
    }

    pub fn reset(&mut self) {
        self.last_tick = current_time();
    }

    pub fn set_interval(&mut self, interval: Duration) {
        self.interval = interval.as_secs_f64() / 1000.;
    }
}
