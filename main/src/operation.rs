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
    let (x, y) = {
        loop {
            let res = Random::get_2d(0..io.n);
            if res != (0, 0) {
                break res;
            }
        }
    };
    let t = *Random::get_item(&state.map[x][y]);
    let (nx, ny) = (|| {
        for d in DIRECTIONS {
            if let Some((nx, ny)) = io.next_pos((x, y), d) {
                if state.map[nx][ny].contains(&(t + 1)) {
                    return (nx, ny);
                }
            }
        }
        unreachable!("generate_add_operation")
    })();

    let d = data.generate_walk(state, (x, y), (nx, ny));
    AddOperation { t, d }
}
