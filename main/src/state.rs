// --- bandle on ---
use crate::direction::*;
use crate::io::*;
use crate::operation::*;
// --- bandle off ---

pub struct State {
    pub d: Vec<Direction>,
    // map[i][j] は空ではないことが保証される
    pub map: Vec<Vec<Vec<usize>>>,
    pub score_map: Vec<Vec<usize>>,
    pub score: usize,
}

#[derive(Debug)]
pub enum Error {
    TooLong,
    CannotMove,
    NotGoal,
    NotVisited,
}

impl State {
    pub fn new(io: &IO, d: Vec<Direction>) -> Result<Self, Error> {
        let l = d.len();

        if l > 100000 {
            return Err(Error::TooLong);
        }

        let map = {
            let mut map = vec![vec![vec![]; io.n]; io.n];
            let mut now = (0, 0);
            for (t, &d) in d.iter().enumerate() {
                if let Some(nxt) = io.next_pos(now, d) {
                    now = nxt;
                    map[now.0][now.1].push(t);
                } else {
                    return Err(Error::CannotMove);
                }
            }
            if now != (0, 0) {
                return Err(Error::NotGoal);
            }
            map
        };

        let score_map = {
            let mut score_map = vec![vec![0; io.n]; io.n];
            for i in 0..io.n {
                for j in 0..io.n {
                    if map[i][j].is_empty() {
                        return Err(Error::NotVisited);
                    }
                    for (k, &t) in map[i][j].iter().enumerate() {
                        let diff = if k == map[i][j].len() - 1 {
                            map[i][j][0] + l - t
                        } else {
                            map[i][j][k + 1] - t
                        };
                        score_map[i][j] += (diff - 1) * diff / 2 * io.d[i][j];
                    }
                }
            }
            score_map
        };

        let score = score_map
            .iter()
            .map(|row| row.iter().sum::<usize>())
            .sum::<usize>()
            / l;

        Ok(Self {
            d,
            map,
            score_map,
            score,
        })
    }

    pub fn convert_to_string(&self) -> String {
        self.d.iter().map(|&d| d.to_char()).collect()
    }

    pub fn apply(&self, io: &IO, operation: &Operation) -> Result<State, Error> {
        match operation {
            Operation::Add(op) => self.apply_add(io, op),
            Operation::Del(op) => self.apply_del(io, op),
            Operation::Tie(op) => self.apply_tie(io, op),
        }
    }

    fn apply_add(&self, io: &IO, operation: &AddOperation) -> Result<State, Error> {
        let (t, d) = (operation.t, &operation.d);
        let mut new_d = vec![];
        for i in 0..self.d.len() {
            if i == t + 1 {
                new_d.extend_from_slice(&d);
            } else {
                new_d.push(self.d[i]);
            }
        }
        State::new(io, new_d)
    }

    fn apply_del(&self, io: &IO, operation: &DelOperation) -> Result<State, Error> {
        let (l, r, d) = (operation.l, operation.r, operation.d);
        let mut new_d = vec![];
        for i in 0..self.d.len() {
            if !(l + 1 <= i && i <= r) {
                new_d.push(self.d[i]);
            }
            if i == l + 1 {
                new_d.push(d);
            }
        }

        // let mut now = (0, 0);
        // for &d in &new_d {
        //     if let Some(nxt) = io.next_pos(now, d) {
        //         now = nxt;
        //     } else {
        //         unreachable!("State::apply_del cannot move");
        //     }
        // }

        // if now != (0, 0) {
        //     unreachable!("State::apply_del now != (0, 0)");
        // }

        State::new(io, new_d)
    }

    pub fn apply_tie(&self, io: &IO, operation: &TieOperation) -> Result<State, Error> {
        let mut new_d = vec![];
        for _ in 0..operation.count {
            new_d.extend_from_slice(&self.d);
        }
        State::new(io, new_d)
    }
}
