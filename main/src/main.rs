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
            if let Some((ni, nj)) = self.io.next_pos((i, j), d) {
                if self.visited[ni][nj] {
                    continue;
                }
                res.push(d);
                self.dfs(ni, nj, res);
                res.push(d.opposite());
            } else {
                continue;
            }
        }
    }

    fn solve(&mut self) {
        let mut res = State::new();
        self.dfs(0, 0, &mut res);
        // eprintln!("{}", res.judge(&self.io).unwrap());
        self.io.output(&res);
    }
}

// Vec<Vec<usize>> で、 (i, j) に x 回(うまくならして)訪れるようなルートの貪欲 を求めるようにして、
// この x 回訪れる パートを焼く...など...？
// ルート長さを 5000 とすると 1000回くらいは焼けそう、もっと焼きたいけど

// TODO: Vec<Vec<Vec<Vec<usize>>>> で、 i, j を始点としたときの ni, nj の最短距離 を求めておく
// これにより、x, y に行きたいときに [x][y]を見つつ、短くなる方面へ進めばよい
// 行けるかのチェックをするべき

fn main() {
    let io = IO::new();
    let mut solver = Solver::new(io);
    solver.solve();
}
