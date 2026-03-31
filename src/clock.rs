use tokio::time::{Duration, sleep};

use chrono::Utc;

pub fn current_time_seconds() -> f64 {
    let now = Utc::now();
    now.timestamp() as f64 + now.timestamp_subsec_micros() as f64 / 1e+6
}

#[derive(Debug)]
pub struct PlockClock {
    last_tick_time: f64,
    pub last_dt: f64,
}

impl PlockClock {
    pub fn new(initial_dt: f64) -> Self {
        Self {
            last_tick_time: 0f64,
            last_dt: initial_dt,
        }
    }

    /// Waiting enough time according to passed tps value
    /// if a tps is negative value, doesn't wait, only update
    pub fn tick(self, tps: f64) -> Self {
        let mut current_time = current_time_seconds();

        if tps.is_sign_positive() {
            while current_time - self.last_tick_time < 1.0 / tps {
                current_time = current_time_seconds();
            }
        }

        Self {
            last_dt: current_time - self.last_tick_time,
            last_tick_time: current_time,
        }
    }

    /// Waiting enough time asynchronously according to passed tps value
    /// if a tps is negative value, doesn't wait, only update
    pub async fn tick_async(self, tps: f64) -> Self {
        let mut current_time = current_time_seconds();
        let target_interval = 1.0 / tps;

        if tps.is_sign_positive() {
            let elapsed = current_time - self.last_tick_time;
            let remaining = target_interval - elapsed;

            if remaining > 0.0 {
                sleep(Duration::from_secs_f64(remaining)).await;
                current_time = current_time_seconds();
            }
        }

        Self {
            last_dt: current_time - self.last_tick_time,
            last_tick_time: current_time,
        }
    }
}
