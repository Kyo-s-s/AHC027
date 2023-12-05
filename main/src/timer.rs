use std::time::SystemTime;

// --- bandle on ---
use crate::random::*;
use crate::state::*;
// --- bandle off ---

pub struct Timer {
    start: SystemTime,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start: SystemTime::now(),
        }
    }

    pub fn get_time(&self) -> f64 {
        let elapsed = self.start.elapsed().expect("Time elapsed failed");
        let elapsed_secs = elapsed.as_secs() as f64;
        let elapsed_micros = elapsed.subsec_micros() as f64;
        elapsed_secs + elapsed_micros / 1_000_000.0
    }

    pub fn force_next(&self, current: &State, next: &State) -> bool {
        let current_score = current.score;
        let next_score = next.score;
        let temp = START_TEMP + (END_TEMP - START_TEMP) * self.get_time() / TL;
        let probability = ((current_score - next_score) / temp).exp();
        probability > Random::get_f()
    }
}

// pub const TL: f64 = 9.9;
// pub const TL: f64 = 4.9;
pub const TL: f64 = 1.9;
// pub const TL: f64 = 0.0;

const START_TEMP: f64 = 100.0;
const END_TEMP: f64 = 0.1;
