#[derive(Debug, Clone, Copy)]
pub enum Direction {
    U = 0,
    R = 1,
    D = 2,
    L = 3,
}

pub const DIRECTIONS: [Direction; 4] = [Direction::U, Direction::R, Direction::D, Direction::L];
const DIRECTION_CHARS: [char; 4] = ['U', 'R', 'D', 'L'];
const DIRECTION_OFFSETS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

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

    pub fn random_directions() -> [Direction; 4] {
        let mut directions = DIRECTIONS;

        directions
    }
}
