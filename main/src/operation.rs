// --- bandle on ---
use crate::data::*;
use crate::direction::*;
use crate::io::*;
use crate::random::*;
use crate::state::*;
use crate::timer::*;
// --- bandle off ---

pub enum Operation {
    Del(DelOperation),
    DelAdd(DelAddOperation),
    Tie(TieOperation),
}

pub fn generate_operation(timer: &Timer, state: &State, io: &IO, data: &Data) -> Operation {
    if Random::get(0..1000) < 1 {
        return Operation::Tie(generate_tie_operation(state, io));
    }
    let t = timer.get_time() / TL;
    let r = Random::get_f();
    if r < t && state.low_routes.len() > 50 {
        // 時間経過でDelAddが選ばれるように
        // 削除区間が決まっちゃって...みたいな？
        Operation::DelAdd(generate_del_add_operation(state, io, data))
    } else {
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

pub struct DelAddOperation {
    pub l: usize,
    pub r: usize,
    pub d: Vec<Direction>,
}

pub fn generate_del_add_operation(state: &State, _io: &IO, data: &Data) -> DelAddOperation {
    let route = Random::get_item(&state.low_routes);
    let path = data.generate_path(state, route.start, route.goal);
    return DelAddOperation {
        l: route.t,
        r: route.nt,
        d: path.d,
    };
}

pub struct TieOperation {
    pub count: usize,
}

pub fn generate_tie_operation(state: &State, _io: &IO) -> TieOperation {
    let l = state.d.len();
    // TODO: len
    if l * 2 > 10000 {
        return TieOperation { count: 1 };
    }
    TieOperation { count: 2 }
}
