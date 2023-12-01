// --- bandle on ---
use crate::direction::*;
use crate::state::*;
// --- bandle off ---

use std::process::exit;

pub struct IO {
    pub n: usize,
    map: Vec<Vec<u8>>,
    pub d: Vec<Vec<usize>>,
}

impl IO {
    pub fn new() -> Self {
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

            for j in 0..n {
                map[0][j] &= !(1 << Direction::U as u8);
                map[n - 1][j] &= !(1 << Direction::D as u8);
            }

            for i in 0..n {
                map[i][0] &= !(1 << Direction::L as u8);
                map[i][n - 1] &= !(1 << Direction::R as u8);
            }
            map
        };

        Self { n, map, d }
    }

    // 枠外に出る場合もここで判定
    pub fn check(&self, i: usize, j: usize, d: Direction) -> bool {
        self.map[i][j] & (1 << d as u8) != 0
    }

    pub fn output(&self, res: &State) {
        let ans = res.convert_to_string();
        println!("{}", ans);
        exit(0);
    }
}
