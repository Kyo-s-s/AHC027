// --- bandle on ---
use crate::direction::*;
use crate::io::*;
// --- bandle off ---

pub struct Data {
    dist: Vec<Vec<Vec<Vec<usize>>>>,
}

impl Data {
    pub fn new(io: &IO) -> Self {
        let dist = {
            let mut dist = vec![vec![vec![vec![usize::MAX; io.n]; io.n]; io.n]; io.n];
            for si in 0..io.n {
                for sj in 0..io.n {
                    let mut q = std::collections::VecDeque::new();
                    q.push_back((si, sj));
                    dist[si][sj][si][sj] = 0;
                    while let Some((i, j)) = q.pop_front() {
                        for d in DIRECTIONS {
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

        Self { dist }
    }
}
