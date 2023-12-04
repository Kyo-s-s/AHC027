// --- bandle on ---
use crate::data::*;
use crate::direction::*;
use crate::io::*;
use crate::random::*;
use crate::state::*;
// --- bandle off ---

pub struct AddOperation {
    pub t: usize,
    pub d: Vec<Direction>,
}

pub fn generate_add_operation(state: &State, io: &IO, data: &Data) -> AddOperation {
    let (t, start, goal) = (|| loop {
        let start = Random::get_2d(0..io.n);
        let t = *Random::get_item(&state.map[start.0][start.1]);
        if t == state.d.len() - 1 {
            continue;
        }
        for d in Direction::all() {
            if let Some(goal) = io.next_pos(start, d) {
                if state.map[goal.0][goal.1].contains(&(t + 1)) {
                    return (t, start, goal);
                }
            }
        }
        unimplemented!("generate_add_operation")
    })();
    let d = data.generate_walk(state, start, goal);
    AddOperation { t, d }
}
