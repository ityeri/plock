use std::thread::sleep;
use std::time::Duration;
use tokio::time as atime;

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
    pub fn default() -> Self {
        Self {
            last_tick_time: 0.0,
            last_dt: 0.0,
        }
    }

    pub fn new(initial_dt: f64) -> Self {
        Self {
            last_tick_time: 0.0,
            last_dt: initial_dt,
        }
    }

    /// Get time initialized clock.
    /// It usable when loop just before
    pub fn initialized(self) -> Self {
        return Self {
            last_tick_time: current_time_seconds(),
            last_dt: self.last_dt,
        };
    }

    /// Waiting enough time according to passed tps value.
    /// if a tps is negative value, doesn't wait, only update
    pub fn tick(self, tps: f64) -> Self {
        let current_time = current_time_seconds();

        if tps.is_sign_positive() {
            let target_interval = 1.0 / tps;
            let elapsed = current_time - self.last_tick_time;
            let remaining = target_interval - elapsed;

            if 0.0 < remaining {
                sleep(Duration::from_secs_f64(remaining));
            }
        }

        let after_tick_time = current_time_seconds();

        Self {
            last_tick_time: after_tick_time,
            last_dt: after_tick_time - self.last_tick_time,
        }
    }

    /// Waiting enough time asynchronously according to passed tps value (with runtime tokio).
    /// if a tps is negative value, doesn't wait, only update
    pub async fn atick(self, tps: f64) -> Self {
        let current_time = current_time_seconds();

        if tps.is_sign_positive() {
            let target_interval = 1.0 / tps;
            let elapsed = current_time - self.last_tick_time;
            let remaining = target_interval - elapsed;

            if 0.0 < remaining {
                atime::sleep(atime::Duration::from_secs_f64(remaining)).await;
            }
        }

        let after_tick_time = current_time_seconds();

        Self {
            last_tick_time: after_tick_time,
            last_dt: after_tick_time - self.last_tick_time,
        }
    }
}
