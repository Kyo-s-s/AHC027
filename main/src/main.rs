use std::{process::exit, str::Chars};

#[derive(Debug, Clone, Copy)]
enum Direction {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Self::U => Self::D,
            Self::R => Self::L,
            Self::D => Self::U,
            Self::L => Self::R,
        }
    }
}

const DIRECTIONS: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];
const DIRECTION_CHARS: [char; 4] = ['U', 'R', 'D', 'L'];

const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

struct IO {
    n: usize,
    map: Vec<Vec<u8>>,
    d: Vec<Vec<usize>>,
}

impl IO {
    fn new() -> Self {
        proconio::input! {
            n: usize,
            h: [String; n - 1],
            v: [String; n],
            d: [[usize; n]; n],
        }

        let h = h
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let v = v
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let map = {
            let mut map = vec![vec![(1 << 4) - 1; n]; n];
            for i in 0..n - 1 {
                for j in 0..n {
                    if h[i][j] == 1 {
                        map[i][j] &= !(1 << Direction::D as u8);
                        map[i + 1][j] &= !(1 << Direction::U as u8);
                    }
                }
            }
            for i in 0..n {
                for j in 0..n - 1 {
                    if v[i][j] == 1 {
                        map[i][j] &= !(1 << Direction::R as u8);
                        map[i][j + 1] &= !(1 << Direction::L as u8);
                    }
                }
            }
            map
        };

        Self { n, map, d }
    }

    fn check(&self, i: usize, j: usize, d: Direction) -> bool {
        self.map[i][j] & (1 << d as u8) != 0
    }

    fn output(&self, res: &Vec<Direction>) {
        let ans = res
            .iter()
            .map(|&d| DIRECTION_CHARS[d as usize])
            .collect::<String>();
        println!("{}", ans);
        exit(0);
    }
}

struct Solver {
    io: IO,
    visited: Vec<Vec<bool>>,
}

impl Solver {
    fn new(io: IO) -> Self {
        let visited = vec![vec![false; io.n]; io.n];
        Self { io, visited }
    }

    fn dfs(&mut self, i: usize, j: usize, res: &mut Vec<Direction>) {
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
        let mut res = vec![];
        self.dfs(0, 0, &mut res);
        self.io.output(&res);
    }
}

fn main() {
    let io = IO::new();
    let mut solver = Solver::new(io);
    solver.solve();
}
