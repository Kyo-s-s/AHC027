// --- bandle on ---
// path: direction.rs
mod direction;
use direction::*;
// path: state.rs
mod state;
use state::*;
// path: io.rs
mod io;
use io::*;
// --- bandle off ---

struct Solver {
    io: IO,
    visited: Vec<Vec<bool>>,
}

impl Solver {
    fn new(io: IO) -> Self {
        let visited = vec![vec![false; io.n]; io.n];
        Self { io, visited }
    }

    fn dfs(&mut self, i: usize, j: usize, res: &mut State) {
        self.visited[i][j] = true;
        for d in DIRECTIONS {
            if !self.io.check(i, j, d) {
                continue;
            }
            let (di, dj) = d.to_offset();
            let ni = (i as i32 + di) as usize;
            let nj = (j as i32 + dj) as usize;
            if self.visited[ni][nj] {
                continue;
            }
            res.push(d);
            self.dfs(ni, nj, res);
            res.push(d.opposite());
        }
    }

    fn solve(&mut self) {
        let mut res = State::new();
        self.dfs(0, 0, &mut res);
        self.io.output(&res);
    }
}

// Vec<Vec<usize>> で、 (i, j) に x 回(うまくならして)訪れるようなルートの貪欲 を求めるようにして、
// この x 回訪れる パートを焼く...など...？
// ルート長さを 5000 とすると 1000回くらいは焼けそう、もっと焼きたいけど

fn main() {
    let io = IO::new();
    let mut solver = Solver::new(io);
    solver.solve();
}
