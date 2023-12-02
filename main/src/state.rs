// --- bandle on ---
use crate::direction::*;
use crate::io::*;
// --- bandle off ---

pub struct State {
    d: Vec<Direction>,
}

impl State {
    pub fn new() -> Self {
        Self { d: vec![] }
    }

    pub fn push(&mut self, d: Direction) {
        self.d.push(d);
    }

    pub fn convert_to_string(&self) -> String {
        self.d.iter().map(|&d| d.to_char()).collect()
    }

    pub fn judge(&self, io: &IO) -> Option<usize> {
        let l = self.d.len();
        let mut map = vec![vec![vec![]; io.n]; io.n];
        let mut now = (0, 0);
        for t in 0..l {
            let d = self.d[t];
            if let Some(nxt) = io.next_pos(now, d) {
                now = nxt;
                map[now.0][now.1].push(t);
            } else {
                return None;
            }
        }

        let mut score = 0;
        for i in 0..io.n {
            for j in 0..io.n {
                let v = &mut map[i][j];
                let cnt = v.len();
                if cnt == 0 {
                    return None;
                }
                v.push(v[0] + l);
                let mut add = 0;
                for k in 0..cnt {
                    let diff = v[k + 1] - v[k];
                    add += (diff - 1) * diff / 2 * io.d[i][j];
                }
                score += add;
            }
        }
        Some(score / l)
    }
}
