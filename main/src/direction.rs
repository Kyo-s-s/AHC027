// --- bandle on ---
use crate::random::*;
// --- bandle off ---

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

const DIRECTIONS: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];
const DIRECTION_CHARS: [char; 4] = ['U', 'R', 'D', 'L'];
const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const DIRECTIONS_ALL: [[Direction; 4]; 24] = [
    [Direction::U, Direction::R, Direction::D, Direction::L],
    [Direction::U, Direction::R, Direction::L, Direction::D],
    [Direction::U, Direction::D, Direction::R, Direction::L],
    [Direction::U, Direction::D, Direction::L, Direction::R],
    [Direction::U, Direction::L, Direction::R, Direction::D],
    [Direction::U, Direction::L, Direction::D, Direction::R],
    [Direction::R, Direction::U, Direction::D, Direction::L],
    [Direction::R, Direction::U, Direction::L, Direction::D],
    [Direction::R, Direction::D, Direction::U, Direction::L],
    [Direction::R, Direction::D, Direction::L, Direction::U],
    [Direction::R, Direction::L, Direction::U, Direction::D],
    [Direction::R, Direction::L, Direction::D, Direction::U],
    [Direction::D, Direction::U, Direction::R, Direction::L],
    [Direction::D, Direction::U, Direction::L, Direction::R],
    [Direction::D, Direction::R, Direction::U, Direction::L],
    [Direction::D, Direction::R, Direction::L, Direction::U],
    [Direction::D, Direction::L, Direction::U, Direction::R],
    [Direction::D, Direction::L, Direction::R, Direction::U],
    [Direction::L, Direction::U, Direction::R, Direction::D],
    [Direction::L, Direction::U, Direction::D, Direction::R],
    [Direction::L, Direction::R, Direction::U, Direction::D],
    [Direction::L, Direction::R, Direction::D, Direction::U],
    [Direction::L, Direction::D, Direction::U, Direction::R],
    [Direction::L, Direction::D, Direction::R, Direction::U],
];

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::U => Self::D,
            Self::R => Self::L,
            Self::D => Self::U,
            Self::L => Self::R,
        }
    }

    pub fn to_offset(&self) -> (i32, i32) {
        DIRECTION_OFFSETS[*self as usize]
    }

    pub fn to_char(&self) -> char {
        DIRECTION_CHARS[*self as usize]
    }

    pub fn all() -> [Direction; 4] {
        DIRECTIONS
    }

    pub fn random() -> [Direction; 4] {
        DIRECTIONS_ALL[Random::get(0..24)]
    }
}
