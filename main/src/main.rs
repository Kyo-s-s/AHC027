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
        let state = State::new(&self.io, res).unwrap();
        let tie_op = generate_tie_operation(&state);
        let mut state = state
            .apply_tie(self.io, &tie_op)
            .unwrap()
            .apply_tie(self.io, &tie_op)
            .unwrap();

        let mut gen = (0, 0);
        let mut app = (0, 0);
        let mut acc = (0, 0);

        let count = |op: &Operation, cnt: (usize, usize)| match op {
            Operation::Del(_) => (cnt.0 + 1, cnt.1),
            Operation::Tie(_) => (cnt.0, cnt.1 + 1),
        };

        while self.timer.get_time() < TL {
            let op = generate_operation(&state, &self.io, &self.data);
            gen = count(&op, gen);
            match state.apply(self.io, &op) {
                Ok(new_state) => {
                    app = count(&op, app);
                    if self.timer.force_next(&state, &new_state) {
                        acc = count(&op, acc);
                        state = new_state;
                    }
                }
                _ => {}
            }
        }

        // eprintln!(
        //     "add: {} / {} / {}, del: {} / {} / {}, tie: {} / {} / {}",
        //     gen.0, app.0, acc.0, gen.1, app.1, acc.1, gen.2, app.2, acc.2
        // );

        self.io.output(&state);
    }
}

fn main() {
    let timer = Timer::new();
    let io = IO::new();
    let data = Data::new(&io);
    let mut solver = Solver::new(timer, &io, data);
    solver.solve()
}
