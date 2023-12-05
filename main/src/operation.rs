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
    Tie(TieOperation),
}

pub fn generate_operation(state: &State, io: &IO, data: &Data) -> Operation {
    let x = Random::get(0..1000);
    if x == 0 {
        Operation::Tie(generate_tie_operation())
    } else if x < 1 {
        // しない
        Operation::Add(generate_add_operation(state, io, data))
    } else {
        // d が小さいのに複数回来ているセルがある、なんでだろう　遷移を見直す
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
    let d = data.generate_walk(state, t, start, goal);
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
                if state.map[goal.0][goal.1].contains(&(t + 1))
                    || state.map[goal.0][goal.1].contains(&(t - 1))
                {
                    continue;
                }
                let nt = state.map[goal.0][goal.1]
                    .iter()
                    .map(|&x| (x, (x as i32 - t as i32).abs()))
                    .min_by_key(|&(_, d)| d)
                    .unwrap()
                    .0;

                let l = t.min(nt);
                let r = t.max(nt);
                let d = if t < nt { d } else { d.opposite() };

                return DelOperation { l, r, d };
            }
        }
    }
}

pub struct TieOperation {
    pub count: usize,
}

fn generate_tie_operation() -> TieOperation {
    TieOperation { count: 2 }
}
