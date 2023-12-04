// --- bandle on ---
// path: timer.rs
mod timer;
use timer::*;
// path: random.rs
mod random;
// path: direction.rs
mod direction;
use direction::*;
// path: state.rs
mod state;
use state::*;
// path: io.rs
mod io;
use io::*;
// path: data.rs
mod data;
use data::*;
// path: operation.rs
mod operation;
use operation::*;
// --- bandle off ---

struct Solver<'a> {
    timer: Timer,
    io: &'a IO,
    data: Data<'a>,
    visited: Vec<Vec<bool>>,
}

impl<'a> Solver<'a> {
    fn new(timer: Timer, io: &'a IO, data: Data<'a>) -> Self {
        let visited = vec![vec![false; io.n]; io.n];
        Self {
            timer,
            io,
            data,
            visited,
        }
    }

    fn dfs(&mut self, i: usize, j: usize, res: &mut Vec<Direction>) {
        self.visited[i][j] = true;
        for d in Direction::all() {
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
        let mut res = vec![];
        self.dfs(0, 0, &mut res);
        // eprintln!("{}", res.judge(&self.io).unwrap());
        let mut state = State::new(&self.io, res).unwrap();

        let (mut add, mut del) = (0, 0);
        while self.timer.get_time() < TL {
            let op = generate_operation(&state, &self.io, &self.data);
            let x = match op {
                Operation::Add(_) => 1,
                Operation::Del(_) => 2,
            };
            if let Some(new_state) = state.apply(self.io, op) {
                if x == 1 {
                    add += 1;
                } else {
                    del += 1;
                }
                if new_state.score < state.score {
                    state = new_state;
                }
            }
        }

        // eprintln!("add: {}, del: {}", add, del);
        self.io.output(&state);
    }
}

// Vec<Vec<usize>> で、 (i, j) に x 回(うまくならして)訪れるようなルートの貪欲 を求めるようにして、
// この x 回訪れる パートを焼く...など...？
// ルート長さを 5000 とすると 1000回くらいは焼けそう、もっと焼きたいけど

// TODO: Vec<Vec<Vec<Vec<usize>>>> で、 i, j を始点としたときの ni, nj の最短距離 を求めておく
// これにより、x, y に行きたいときに [x][y]を見つつ、短くなる方面へ進めばよい
// 行けるかのチェックをするべき

fn main() {
    let timer = Timer::new();
    let io = IO::new();
    let data = Data::new(&io);
    let mut solver = Solver::new(timer, &io, data);
    solver.solve()
}
