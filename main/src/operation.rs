// --- bandle on ---
use crate::data::*;
use crate::direction::*;
use crate::io::*;
use crate::random::*;
use crate::state::*;
// --- bandle off ---

pub enum Operation {
    Add(AddOperation),
    Del(DelOperation),
}

pub fn generate_operation(state: &State, io: &IO, data: &Data) -> Operation {
    let x = Random::get(0..100);
    if x < 50 {
        Operation::Add(generate_add_operation(state, io, data))
    } else {
        Operation::Del(generate_del_operation(state, io))
    }
}

pub struct AddOperation {
    pub t: usize,
    pub d: Vec<Direction>,
}

fn generate_add_operation(state: &State, io: &IO, data: &Data) -> AddOperation {
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

pub struct DelOperation {
    // [l, r] を削除、 d を入れる
    pub l: usize,
    pub r: usize,
    pub d: Direction,
}

fn generate_del_operation(state: &State, io: &IO) -> DelOperation {
    loop {
        let start = Random::get_2d(0..io.n);
        let t = *Random::get_item(&state.map[start.0][start.1]);
        for d in Direction::random() {
            if let Some(goal) = io.next_pos(start, d) {
                let nt = *Random::get_item(&state.map[goal.0][goal.1]);
                return DelOperation {
                    l: t.min(nt),
                    r: t.max(nt) - 1,
                    d,
                };
            }
        }
    }
}
