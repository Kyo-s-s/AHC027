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
            let (di, dj) = DIRECTION_OFFSETS[d as usize];
            let ni = i as i32 + di;
            let nj = j as i32 + dj;
            if (0..self.io.n as i32).contains(&ni) && (0..self.io.n as i32).contains(&nj) {
                let (ni, nj) = (ni as usize, nj as usize);
                if self.visited[ni][nj] || !self.io.check(i, j, d) {
                    continue;
                }
                res.push(d);
                self.dfs(ni, nj, res);
                res.push(d.opposite());
            }
        }
    }

    fn solve(&mut self) {
        let mut res = State::new();
        self.dfs(0, 0, &mut res);
        self.io.output(&res);
    }
}

fn main() {
    let io = IO::new();
    let mut solver = Solver::new(io);
    solver.solve();
}
