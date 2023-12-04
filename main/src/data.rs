// --- bandle on ---
use crate::direction::*;
use crate::io::*;
use crate::random::*;
use crate::state::*;
// --- bandle off ---

struct Walk {
    d: Vec<Direction>,
    start: (usize, usize),
    pos: (usize, usize),
}

impl Walk {
    fn new(pos: (usize, usize)) -> Self {
        Self {
            d: vec![],
            start: pos,
            pos,
        }
    }

    fn add(&mut self, io: &IO, d: Direction) {
        self.d.push(d);
        if let Some(npos) = io.next_pos(self.pos, d) {
            self.pos = npos;
        } else {
            unreachable!("Walk::add");
        }
    }

    fn connect(&mut self, other: Self) {
        if self.pos != other.start {
            unreachable!("Walk::connect");
        }
        self.d.extend(other.d);
        self.pos = other.pos;
    }
}

pub struct Data<'a> {
    io: &'a IO,
    dist: Vec<Vec<Vec<Vec<usize>>>>,
}

impl<'a> Data<'a> {
    pub fn new(io: &'a IO) -> Self {
        // dist の計算に N = 40 のときで 50ms くらい 前処理なので許容範囲？
        let dist = {
            let mut dist = vec![vec![vec![vec![usize::MAX; io.n]; io.n]; io.n]; io.n];
            for si in 0..io.n {
                for sj in 0..io.n {
                    let mut q = std::collections::VecDeque::new();
                    q.push_back((si, sj));
                    dist[si][sj][si][sj] = 0;
                    while let Some((i, j)) = q.pop_front() {
                        for d in Direction::all() {
                            if let Some((ni, nj)) = io.next_pos((i, j), d) {
                                if dist[si][sj][ni][nj] == usize::MAX {
                                    dist[si][sj][ni][nj] = dist[si][sj][i][j] + 1;
                                    q.push_back((ni, nj));
                                }
                            }
                        }
                    }
                }
            }
            dist
        };

        Self { io, dist }
    }

    // TODO: 不要なので消す、いい感じにする
    fn generate_path(&self, start: (usize, usize), goal: (usize, usize)) -> Walk {
        let mut res = Walk::new(start);
        let dist = &self.dist[goal.0][goal.1];
        while res.pos != goal {
            let d = (|| {
                for d in Direction::random() {
                    if let Some((nx, ny)) = self.io.next_pos(res.pos, d) {
                        if dist[nx][ny] < dist[res.pos.0][res.pos.1] {
                            return d;
                        }
                    }
                }
                unreachable!("generate_path");
            })();
            res.add(self.io, d);
        }
        res
    }

    pub fn generate_walk(
        &self,
        state: &State,
        start: (usize, usize),
        goal: (usize, usize),
    ) -> Vec<Direction> {
        // とりあえず雑に構築　ここを工夫することでスコアが上がる state に依りたいので引数にしておく
        let rand = Random::get_2d(0..self.io.n);
        let mut res = self.generate_path(start, rand);
        res.connect(self.generate_path(rand, goal));
        res.d
    }
}
