// --- bandle on ---
use crate::io::*;
// --- bandle off ---

pub struct Data<'a> {
    io: &'a IO,
}

impl<'a> Data<'a> {
    pub fn new(io: &'a IO) -> Self {
        Self { io }
    }
}
