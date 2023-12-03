// --- bandle on ---
use crate::data::*;
use crate::direction::*;
use crate::io::*;
use crate::state::*;
// --- bandle off ---

pub struct Operation {
    t: usize,
    d: Vec<Direction>,
}

pub fn generate_add_operation(state: &State, io: &IO, data: &Data) -> Operation {
    todo!();
}
