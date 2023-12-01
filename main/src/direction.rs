#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Self::U => Self::D,
            Self::R => Self::L,
            Self::D => Self::U,
            Self::L => Self::R,
        }
    }
}

pub const DIRECTIONS: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];
pub const DIRECTION_CHARS: [char; 4] = ['U', 'R', 'D', 'L'];

pub const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
