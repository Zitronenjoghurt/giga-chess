use std::error::Error;
use std::fmt::Display;

pub const A1: u8 = 0;
pub const B1: u8 = 1;
pub const C1: u8 = 2;
pub const D1: u8 = 3;
pub const E1: u8 = 4;
pub const F1: u8 = 5;
pub const G1: u8 = 6;
pub const H1: u8 = 7;
pub const A2: u8 = 8;
pub const B2: u8 = 9;
pub const C2: u8 = 10;
pub const D2: u8 = 11;
pub const E2: u8 = 12;
pub const F2: u8 = 13;
pub const G2: u8 = 14;
pub const H2: u8 = 15;
pub const A3: u8 = 16;
pub const B3: u8 = 17;
pub const C3: u8 = 18;
pub const D3: u8 = 19;
pub const E3: u8 = 20;
pub const F3: u8 = 21;
pub const G3: u8 = 22;
pub const H3: u8 = 23;
pub const A4: u8 = 24;
pub const B4: u8 = 25;
pub const C4: u8 = 26;
pub const D4: u8 = 27;
pub const E4: u8 = 28;
pub const F4: u8 = 29;
pub const G4: u8 = 30;
pub const H4: u8 = 31;
pub const A5: u8 = 32;
pub const B5: u8 = 33;
pub const C5: u8 = 34;
pub const D5: u8 = 35;
pub const E5: u8 = 36;
pub const F5: u8 = 37;
pub const G5: u8 = 38;
pub const H5: u8 = 39;
pub const A6: u8 = 40;
pub const B6: u8 = 41;
pub const C6: u8 = 42;
pub const D6: u8 = 43;
pub const E6: u8 = 44;
pub const F6: u8 = 45;
pub const G6: u8 = 46;
pub const H6: u8 = 47;
pub const A7: u8 = 48;
pub const B7: u8 = 49;
pub const C7: u8 = 50;
pub const D7: u8 = 51;
pub const E7: u8 = 52;
pub const F7: u8 = 53;
pub const G7: u8 = 54;
pub const H7: u8 = 55;
pub const A8: u8 = 56;
pub const B8: u8 = 57;
pub const C8: u8 = 58;
pub const D8: u8 = 59;
pub const E8: u8 = 60;
pub const F8: u8 = 61;
pub const G8: u8 = 62;
pub const H8: u8 = 63;

/// A square on the chess board, indexing starts with 0 at A1, 1 at B1 and ends with 63 at H8.
#[derive(Debug, Copy, Clone)]
#[repr(transparent)]
pub struct Square(u8);

impl Square {
    pub const fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn from_file_rank(file: u8, rank: u8) -> Self {
        Self((file - 1) + (rank - 1) * 8)
    }

    pub fn is_white(&self) -> bool {
        (self.get_file() + self.get_rank()) % 2 == 0
    }

    pub fn get_value(&self) -> u8 {
        self.0
    }

    pub fn get_rank(&self) -> u8 {
        (self.0 / 8) + 1
    }

    pub fn get_file(&self) -> u8 {
        (self.0 % 8) + 1
    }

    pub fn is_left_edge(&self) -> bool {
        self.0 % 8 == 0
    }

    pub fn is_right_edge(&self) -> bool {
        self.0 % 8 == 7
    }

    pub fn is_upper_edge(&self) -> bool {
        self.0 >= 56
    }

    pub fn is_lower_edge(&self) -> bool {
        self.0 <= 7
    }

    pub fn index_left(&self) -> Option<u8> {
        if self.is_left_edge() {
            None
        } else {
            Some(self.0 - 1)
        }
    }

    pub fn index_right(&self) -> Option<u8> {
        if self.is_right_edge() {
            None
        } else {
            Some(self.0 + 1)
        }
    }

    pub fn index_up(&self) -> Option<u8> {
        if self.is_upper_edge() {
            None
        } else {
            Some(self.0 + 8)
        }
    }

    pub fn index_down(&self) -> Option<u8> {
        if self.is_lower_edge() {
            None
        } else {
            Some(self.0 - 8)
        }
    }

    pub fn index_up_right(&self) -> Option<u8> {
        if self.is_upper_edge() || self.is_right_edge() {
            None
        } else {
            Some(self.0 + 9)
        }
    }

    pub fn index_up_left(&self) -> Option<u8> {
        if self.is_upper_edge() || self.is_left_edge() {
            None
        } else {
            Some(self.0 + 7)
        }
    }

    pub fn index_down_right(&self) -> Option<u8> {
        if self.is_lower_edge() || self.is_right_edge() {
            None
        } else {
            Some(self.0 - 7)
        }
    }

    pub fn index_down_left(&self) -> Option<u8> {
        if self.is_lower_edge() || self.is_left_edge() {
            None
        } else {
            Some(self.0 - 9)
        }
    }

    pub fn index_jump(&self, file: i8, rank: i8) -> Option<u8> {
        let current_file = self.get_file();
        let current_rank = self.get_rank();
        let new_file = current_file as i8 + file;
        let new_rank = current_rank as i8 + rank;
        if new_file < 1 || new_file > 8 || new_rank < 1 || new_rank > 8 {
            None
        } else {
            Some(Square::from_file_rank(new_file as u8, new_rank as u8).get_value())
        }
    }

    pub fn trace_up(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_up(), |&idx| Square::new(idx).index_up())
    }

    pub fn trace_down(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_down(), |&idx| Square::new(idx).index_down())
    }

    pub fn trace_left(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_left(), |&idx| Square::new(idx).index_left())
    }

    pub fn trace_right(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_right(), |&idx| Square::new(idx).index_right())
    }

    pub fn trace_up_left(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_up_left(), |&idx| {
            Square::new(idx).index_up_left()
        })
    }

    pub fn trace_up_right(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_up_right(), |&idx| {
            Square::new(idx).index_up_right()
        })
    }

    pub fn trace_down_left(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_down_left(), |&idx| {
            Square::new(idx).index_down_left()
        })
    }

    pub fn trace_down_right(&self) -> impl Iterator<Item = u8> {
        std::iter::successors(self.index_down_right(), |&idx| {
            Square::new(idx).index_down_right()
        })
    }

    pub fn get_file_char(&self) -> char {
        match self.get_file() {
            1 => 'A',
            2 => 'B',
            3 => 'C',
            4 => 'D',
            5 => 'E',
            6 => 'F',
            7 => 'G',
            8 => 'H',
            _ => unreachable!(),
        }
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let file = self.get_file();
        let rank = self.get_rank();
        let str = match (file, rank) {
            (1, 1) => "A1".to_string(),
            (1, 2) => "A2".to_string(),
            (1, 3) => "A3".to_string(),
            (1, 4) => "A4".to_string(),
            (1, 5) => "A5".to_string(),
            (1, 6) => "A6".to_string(),
            (1, 7) => "A7".to_string(),
            (1, 8) => "A8".to_string(),
            (2, 1) => "B1".to_string(),
            (2, 2) => "B2".to_string(),
            (2, 3) => "B3".to_string(),
            (2, 4) => "B4".to_string(),
            (2, 5) => "B5".to_string(),
            (2, 6) => "B6".to_string(),
            (2, 7) => "B7".to_string(),
            (2, 8) => "B8".to_string(),
            (3, 1) => "C1".to_string(),
            (3, 2) => "C2".to_string(),
            (3, 3) => "C3".to_string(),
            (3, 4) => "C4".to_string(),
            (3, 5) => "C5".to_string(),
            (3, 6) => "C6".to_string(),
            (3, 7) => "C7".to_string(),
            (3, 8) => "C8".to_string(),
            (4, 1) => "D1".to_string(),
            (4, 2) => "D2".to_string(),
            (4, 3) => "D3".to_string(),
            (4, 4) => "D4".to_string(),
            (4, 5) => "D5".to_string(),
            (4, 6) => "D6".to_string(),
            (4, 7) => "D7".to_string(),
            (4, 8) => "D8".to_string(),
            (5, 1) => "E1".to_string(),
            (5, 2) => "E2".to_string(),
            (5, 3) => "E3".to_string(),
            (5, 4) => "E4".to_string(),
            (5, 5) => "E5".to_string(),
            (5, 6) => "E6".to_string(),
            (5, 7) => "E7".to_string(),
            (5, 8) => "E8".to_string(),
            (6, 1) => "F1".to_string(),
            (6, 2) => "F2".to_string(),
            (6, 3) => "F3".to_string(),
            (6, 4) => "F4".to_string(),
            (6, 5) => "F5".to_string(),
            (6, 6) => "F6".to_string(),
            (6, 7) => "F7".to_string(),
            (6, 8) => "F8".to_string(),
            (7, 1) => "G1".to_string(),
            (7, 2) => "G2".to_string(),
            (7, 3) => "G3".to_string(),
            (7, 4) => "G4".to_string(),
            (7, 5) => "G5".to_string(),
            (7, 6) => "G6".to_string(),
            (7, 7) => "G7".to_string(),
            (7, 8) => "G8".to_string(),
            (8, 1) => "H1".to_string(),
            (8, 2) => "H2".to_string(),
            (8, 3) => "H3".to_string(),
            (8, 4) => "H4".to_string(),
            (8, 5) => "H5".to_string(),
            (8, 6) => "H6".to_string(),
            (8, 7) => "H7".to_string(),
            (8, 8) => "H8".to_string(),
            _ => unreachable!(),
        };
        write!(f, "{}", str)
    }
}

impl TryFrom<&str> for Square {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() != 2 {
            return Err(format!("Invalid square '{value}'").into());
        }

        let file_char = value.chars().nth(0).unwrap();
        let rank_char = value.chars().nth(1).unwrap();

        let file: u8 = match file_char {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
            'E' => 5,
            'F' => 6,
            'G' => 7,
            'H' => 8,
            _ => return Err(format!("Invalid file '{file_char}'").into()),
        };

        let rank: u8 = match rank_char {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => return Err(format!("Invalid rank '{rank_char}'").into()),
        };

        Ok(Square::from_file_rank(file, rank))
    }
}
