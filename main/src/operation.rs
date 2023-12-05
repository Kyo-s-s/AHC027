// --- bandle on ---
use crate::data::*;
use crate::direction::*;
use crate::io::*;
use crate::random::*;
use crate::state::*;
// --- bandle off ---

pub enum Operation {
    Del(DelOperation),
    Tie(TieOperation),
}

pub fn generate_operation(state: &State, io: &IO, _data: &Data) -> Operation {
    let x = Random::get(0..1000);
    if x == 0 {
        Operation::Tie(generate_tie_operation(state))
    } else {
        // d が小さいのに複数回来ているセルがある、なんでだろう　遷移を見直す
        Operation::Del(generate_del_operation(state, io))
    }
}

pub struct DelOperation {
    // [l, r] を削除、 d を入れる
    pub l: usize,
    pub r: usize,
    pub d: Direction,
}

fn generate_del_operation(state: &State, io: &IO) -> DelOperation {
    loop {
        // d が小さいのを start に選びやすくする...など
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

pub fn generate_tie_operation(state: &State) -> TieOperation {
    let l = state.d.len();
    if l * 2 > 10000 {
        return TieOperation { count: 1 };
    }
    TieOperation { count: 2 }
}
