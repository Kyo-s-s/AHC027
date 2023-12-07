// --- bandle on ---
use crate::data::*;
use crate::direction::*;
use crate::io::*;
use crate::operation::*;
use crate::random::*;
// --- bandle off ---

#[derive(Debug, Clone)]
pub struct Route {
    pub t: usize,
    pub start: (usize, usize),
    pub nt: usize,
    pub goal: (usize, usize),
}

impl Route {
    fn new(t: usize, start: (usize, usize)) -> Self {
        Self {
            t,
            start,
            nt: t,
            goal: start,
        }
    }

    fn add(&mut self, io: &IO, d: Direction) {
        if let Some(nxt) = io.next_pos(self.goal, d) {
            self.goal = nxt;
            self.nt += 1;
        } else {
            unreachable!("Route::add");
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub d: Vec<Direction>,
    // map[i][j] は空ではないことが保証される
    pub map: Vec<Vec<Vec<usize>>>,
    pub low_routes: Vec<Route>,
    pub score_map: Vec<Vec<usize>>,
    pub score: f64,
}

#[derive(Debug)]
pub enum Error {
    TooLong,
    CannotMove,
    NotGoal,
    NotVisited,
}

impl State {
    pub fn new(io: &IO, data: &Data, d: Vec<Direction>) -> Result<Self, Error> {
        let l = d.len();

        if l > 100000 {
            return Err(Error::TooLong);
        }

        let (low_routes, map) = {
            let mut low_routes: Vec<Route> = vec![];
            let mut map = vec![vec![vec![]; io.n]; io.n];
            let mut now = (0, 0);
            for (t, &d) in d.iter().enumerate() {
                if let Some(nxt) = io.next_pos(now, d) {
                    map[nxt.0][nxt.1].push(t);
                    if io.d[nxt.0][nxt.1] < Random::get(0..500) {
                        if low_routes.is_empty() || low_routes.last().unwrap().nt != t - 1 {
                            low_routes.push(Route::new(t, nxt));
                        } else if low_routes.last().unwrap().nt == t - 1 {
                            low_routes.last_mut().unwrap().add(io, d);
                        }
                    }
                    now = nxt;
                } else {
                    return Err(Error::CannotMove);
                }
            }
            if now != (0, 0) {
                return Err(Error::NotGoal);
            }
            let low_routes = low_routes
                .iter()
                .filter(|&r| {
                    let d = r.nt - r.t;
                    data.dist(r.start, r.goal) + 3 < d
                })
                .cloned()
                .collect::<Vec<_>>();
            (low_routes, map)
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
            .sum::<usize>() as f64
            / l as f64;

        Ok(Self {
            d,
            map,
            low_routes,
            score_map,
            score,
        })
    }

    pub fn convert_to_string(&self) -> String {
        self.d.iter().map(|&d| d.to_char()).collect()
    }

    pub fn apply(&self, io: &IO, data: &Data, operation: &Operation) -> Result<State, Error> {
        match operation {
            Operation::Del(op) => self.apply_del(io, data, op),
            Operation::DelAdd(op) => self.apply_del_add(io, data, op),
            Operation::Tie(op) => self.apply_tie(io, data, op),
        }
    }

    fn apply_del(&self, io: &IO, data: &Data, operation: &DelOperation) -> Result<State, Error> {
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
        State::new(io, data, new_d)
    }

    fn apply_del_add(
        &self,
        io: &IO,
        data: &Data,
        operation: &DelAddOperation,
    ) -> Result<State, Error> {
        let (l, r, d) = (operation.l, operation.r, &operation.d);
        let mut new_d = vec![];
        for i in 0..self.d.len() {
            if !(l + 1 <= i && i <= r) {
                new_d.push(self.d[i]);
            }
            if i == l + 1 {
                new_d.extend_from_slice(d);
            }
        }
        State::new(io, data, new_d)
    }

    pub fn apply_tie(
        &self,
        io: &IO,
        data: &Data,
        operation: &TieOperation,
    ) -> Result<State, Error> {
        if operation.count == 1 {
            return Ok(self.clone());
        }
        let mut new_d = vec![];
        for _ in 0..operation.count {
            new_d.extend_from_slice(&self.d);
        }
        State::new(io, data, new_d)
    }
}
