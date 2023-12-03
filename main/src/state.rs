use std::vec;

// --- bandle on ---
use crate::direction::*;
use crate::io::*;
use crate::operation::*;
// --- bandle off ---

pub struct State {
    pub d: Vec<Direction>,
    // map[i][j] は空ではないことが保証される
    pub map: Vec<Vec<Vec<usize>>>,
    pub score: usize,
}

impl State {
    pub fn new(io: &IO, d: Vec<Direction>) -> Option<Self> {
        let l = d.len();
        let map = {
            let mut map = vec![vec![vec![]; io.n]; io.n];
            let mut now = (0, 0);
            for (t, &d) in d.iter().enumerate() {
                if let Some(nxt) = io.next_pos(now, d) {
                    now = nxt;
                    map[now.0][now.1].push(t);
                } else {
                    return None;
                }
            }
            map
        };

        let score = {
            let mut score = 0;
            for i in 0..io.n {
                for j in 0..io.n {
                    if map[i][j].is_empty() {
                        return None;
                    }
                    for (k, &t) in map[i][j].iter().enumerate() {
                        let diff = if k == map[i][j].len() - 1 {
                            map[i][j][0] + l - t
                        } else {
                            map[i][j][k + 1] - t
                        };
                        score += (diff - 1) * diff / 2 * io.d[i][j];
                    }
                }
            }
            score / l
        };
        Some(Self { d, map, score })
    }

    pub fn convert_to_string(&self) -> String {
        self.d.iter().map(|&d| d.to_char()).collect()
    }

    pub fn apply_add(&self, io: &IO, operation: AddOperation) -> Option<State> {
        let (t, d) = (operation.t, operation.d);
        let mut new_d = vec![];
        for i in 0..self.d.len() {
            if i == t {
                new_d.extend_from_slice(&d);
            } else {
                new_d.push(self.d[i]);
            }
        }
        State::new(io, new_d)
    }
}
