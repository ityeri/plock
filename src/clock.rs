use chrono::Utc;

pub fn current_time_seconds() -> f64 {
    let now = Utc::now();
    now.timestamp() as f64 + now.timestamp_subsec_micros() as f64 / 1e+6
}

#[derive(Debug)]
pub struct Clock {
    last_tick_time: f64,
    pub last_dt: f64,
}

impl Clock {
    pub fn new(initial_dt: f64) -> Self {
        Self {
            last_tick_time: 0f64,
            last_dt: initial_dt,
        }
    }

    pub fn tick(self, tps: f64) -> Self {
        let mut current_time = current_time_seconds();

        while current_time - self.last_tick_time < 1.0 / tps {
            current_time = current_time_seconds();
        }

        Self {
            last_dt: current_time - self.last_tick_time,
            last_tick_time: current_time,
        }
    }
}
