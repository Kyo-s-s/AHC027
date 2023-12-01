// --- bandle on ---
use crate::direction::*;
// --- bandle off ---

pub struct State {
    d: Vec<Direction>,
}

impl State {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    pub fn push(&mut self, d: Direction) {
        self.d.push(d);
    }

    pub fn convert_to_string(&self) -> String {
        self.d
            .iter()
            .map(|&d| DIRECTION_CHARS[d as usize])
            .collect()
    }
}
