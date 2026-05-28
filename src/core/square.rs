use crate::core::piece::Color;
use crate::error::{FenError, FenResult};
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct Square(u8);

impl Square {
    pub const fn new(index: u8) -> Self {
        assert!(index < 64, "Square index must be 0..64");
        Self(index)
    }

    /// # Safety
    /// Caller guarantees index < 64.
    pub const unsafe fn new_unchecked(index: u8) -> Self {
        Self(index)
    }

    pub const fn try_new(index: u8) -> Option<Self> {
        if index < 64 { Some(Self(index)) } else { None }
    }

    pub const fn index(self) -> u8 {
        self.0
    }

    pub const fn from_file_rank(file: u8, rank: u8) -> Self {
        Self((file - 1) + (rank - 1) * 8)
    }

    pub const fn is_white(&self) -> bool {
        (self.file() + self.rank()) % 2 == 1
    }

    /// 1-based
    pub const fn rank(&self) -> u8 {
        (self.0 / 8) + 1
    }

    /// 1-based
    pub const fn file(&self) -> u8 {
        (self.0 % 8) + 1
    }

    pub const fn is_promotion_square(&self, color: Color) -> bool {
        match color {
            Color::White => self.is_upper_edge(),
            Color::Black => self.is_lower_edge(),
        }
    }

    pub const fn is_any_promotion_square(&self) -> bool {
        self.is_upper_edge() || self.is_lower_edge()
    }

    pub const fn is_pawn_start(&self, color: Color) -> bool {
        match color {
            Color::White => self.0 > 7 && self.0 < 16,
            Color::Black => self.0 > 47 && self.0 < 56,
        }
    }

    pub(crate) const fn pawn_push(self, color: Color) -> Self {
        match color {
            Color::White => self.pp_white(),
            Color::Black => self.pp_black(),
        }
    }

    pub(crate) const fn pp_white(self) -> Self {
        Self(self.0 + 8)
    }

    pub(crate) const fn pp_black(self) -> Self {
        Self(self.0 - 8)
    }

    pub(crate) const fn double_pawn_push(self, color: Color) -> Self {
        match color {
            Color::White => self.dpp_white(),
            Color::Black => self.dpp_black(),
        }
    }

    /// Double pawn push for white
    pub(crate) const fn dpp_white(self) -> Self {
        Self(self.0 + 16)
    }

    /// Double pawn push for black
    pub(crate) const fn dpp_black(self) -> Self {
        Self(self.0 - 16)
    }

    pub const fn is_left_edge(&self) -> bool {
        self.0.is_multiple_of(8)
    }

    pub const fn is_right_edge(&self) -> bool {
        self.0 % 8 == 7
    }

    pub const fn is_upper_edge(&self) -> bool {
        self.0 >= 56
    }

    pub const fn is_lower_edge(&self) -> bool {
        self.0 <= 7
    }

    pub const fn left(&self) -> Option<Square> {
        if self.is_left_edge() {
            None
        } else {
            Some(Square::new(self.0 - 1))
        }
    }

    pub const fn right(&self) -> Option<Square> {
        if self.is_right_edge() {
            None
        } else {
            Some(Square::new(self.0 + 1))
        }
    }

    pub const fn up(&self) -> Option<Square> {
        if self.is_upper_edge() {
            None
        } else {
            Some(Square::new(self.0 + 8))
        }
    }

    pub const fn down(&self) -> Option<Square> {
        if self.is_lower_edge() {
            None
        } else {
            Some(Square::new(self.0 - 8))
        }
    }

    pub const fn up_right(&self) -> Option<Square> {
        if self.is_upper_edge() || self.is_right_edge() {
            None
        } else {
            Some(Square::new(self.0 + 9))
        }
    }

    pub const fn up_left(&self) -> Option<Square> {
        if self.is_upper_edge() || self.is_left_edge() {
            None
        } else {
            Some(Square::new(self.0 + 7))
        }
    }

    pub const fn down_right(&self) -> Option<Square> {
        if self.is_lower_edge() || self.is_right_edge() {
            None
        } else {
            Some(Square::new(self.0 - 7))
        }
    }

    pub const fn down_left(&self) -> Option<Square> {
        if self.is_lower_edge() || self.is_left_edge() {
            None
        } else {
            Some(Square::new(self.0 - 9))
        }
    }

    pub fn jump(&self, file: i8, rank: i8) -> Option<Square> {
        let current_file = self.file();
        let current_rank = self.rank();
        let new_file = current_file as i8 + file;
        let new_rank = current_rank as i8 + rank;
        if !(1..=8).contains(&new_file) || !(1..=8).contains(&new_rank) {
            None
        } else {
            Some(Square::from_file_rank(new_file as u8, new_rank as u8))
        }
    }

    pub fn trace_up(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.up(), |&sq| sq.up())
    }

    pub fn trace_down(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.down(), |&sq| sq.down())
    }

    pub fn trace_left(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.left(), |&sq| sq.left())
    }

    pub fn trace_right(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.right(), |&sq| sq.right())
    }

    pub fn trace_up_left(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.up_left(), |&sq| sq.up_left())
    }

    pub fn trace_up_right(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.up_right(), |&sq| sq.up_right())
    }

    pub fn trace_down_left(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.down_left(), |&sq| sq.down_left())
    }

    pub fn trace_down_right(&self) -> impl Iterator<Item = Square> {
        std::iter::successors(self.down_right(), |&sq| sq.down_right())
    }

    pub fn iter_bottom_top() -> impl Iterator<Item = Square> {
        (0..64).map(Square::new)
    }

    pub fn iter_top_bottom() -> impl Iterator<Item = Square> {
        (1..=8)
            .rev()
            .flat_map(|rank| (1..=8).map(move |file| Square::from_file_rank(file, rank)))
    }

    pub fn iter_indices() -> impl Iterator<Item = u8> {
        0..64
    }

    pub const fn file_char(&self) -> char {
        (b'a' + self.file() - 1) as char
    }

    pub const fn rank_char(&self) -> char {
        (b'0' + self.rank()) as char
    }

    pub const A1: Self = Self(0);
    pub const B1: Self = Self(1);
    pub const C1: Self = Self(2);
    pub const D1: Self = Self(3);
    pub const E1: Self = Self(4);
    pub const F1: Self = Self(5);
    pub const G1: Self = Self(6);
    pub const H1: Self = Self(7);
    pub const A2: Self = Self(8);
    pub const B2: Self = Self(9);
    pub const C2: Self = Self(10);
    pub const D2: Self = Self(11);
    pub const E2: Self = Self(12);
    pub const F2: Self = Self(13);
    pub const G2: Self = Self(14);
    pub const H2: Self = Self(15);
    pub const A3: Self = Self(16);
    pub const B3: Self = Self(17);
    pub const C3: Self = Self(18);
    pub const D3: Self = Self(19);
    pub const E3: Self = Self(20);
    pub const F3: Self = Self(21);
    pub const G3: Self = Self(22);
    pub const H3: Self = Self(23);
    pub const A4: Self = Self(24);
    pub const B4: Self = Self(25);
    pub const C4: Self = Self(26);
    pub const D4: Self = Self(27);
    pub const E4: Self = Self(28);
    pub const F4: Self = Self(29);
    pub const G4: Self = Self(30);
    pub const H4: Self = Self(31);
    pub const A5: Self = Self(32);
    pub const B5: Self = Self(33);
    pub const C5: Self = Self(34);
    pub const D5: Self = Self(35);
    pub const E5: Self = Self(36);
    pub const F5: Self = Self(37);
    pub const G5: Self = Self(38);
    pub const H5: Self = Self(39);
    pub const A6: Self = Self(40);
    pub const B6: Self = Self(41);
    pub const C6: Self = Self(42);
    pub const D6: Self = Self(43);
    pub const E6: Self = Self(44);
    pub const F6: Self = Self(45);
    pub const G6: Self = Self(46);
    pub const H6: Self = Self(47);
    pub const A7: Self = Self(48);
    pub const B7: Self = Self(49);
    pub const C7: Self = Self(50);
    pub const D7: Self = Self(51);
    pub const E7: Self = Self(52);
    pub const F7: Self = Self(53);
    pub const G7: Self = Self(54);
    pub const H7: Self = Self(55);
    pub const A8: Self = Self(56);
    pub const B8: Self = Self(57);
    pub const C8: Self = Self(58);
    pub const D8: Self = Self(59);
    pub const E8: Self = Self(60);
    pub const F8: Self = Self(61);
    pub const G8: Self = Self(62);
    pub const H8: Self = Self(63);
}

pub const A1: Square = Square::A1;
pub const B1: Square = Square::B1;
pub const C1: Square = Square::C1;
pub const D1: Square = Square::D1;
pub const E1: Square = Square::E1;
pub const F1: Square = Square::F1;
pub const G1: Square = Square::G1;
pub const H1: Square = Square::H1;
pub const A2: Square = Square::A2;
pub const B2: Square = Square::B2;
pub const C2: Square = Square::C2;
pub const D2: Square = Square::D2;
pub const E2: Square = Square::E2;
pub const F2: Square = Square::F2;
pub const G2: Square = Square::G2;
pub const H2: Square = Square::H2;
pub const A3: Square = Square::A3;
pub const B3: Square = Square::B3;
pub const C3: Square = Square::C3;
pub const D3: Square = Square::D3;
pub const E3: Square = Square::E3;
pub const F3: Square = Square::F3;
pub const G3: Square = Square::G3;
pub const H3: Square = Square::H3;
pub const A4: Square = Square::A4;
pub const B4: Square = Square::B4;
pub const C4: Square = Square::C4;
pub const D4: Square = Square::D4;
pub const E4: Square = Square::E4;
pub const F4: Square = Square::F4;
pub const G4: Square = Square::G4;
pub const H4: Square = Square::H4;
pub const A5: Square = Square::A5;
pub const B5: Square = Square::B5;
pub const C5: Square = Square::C5;
pub const D5: Square = Square::D5;
pub const E5: Square = Square::E5;
pub const F5: Square = Square::F5;
pub const G5: Square = Square::G5;
pub const H5: Square = Square::H5;
pub const A6: Square = Square::A6;
pub const B6: Square = Square::B6;
pub const C6: Square = Square::C6;
pub const D6: Square = Square::D6;
pub const E6: Square = Square::E6;
pub const F6: Square = Square::F6;
pub const G6: Square = Square::G6;
pub const H6: Square = Square::H6;
pub const A7: Square = Square::A7;
pub const B7: Square = Square::B7;
pub const C7: Square = Square::C7;
pub const D7: Square = Square::D7;
pub const E7: Square = Square::E7;
pub const F7: Square = Square::F7;
pub const G7: Square = Square::G7;
pub const H7: Square = Square::H7;
pub const A8: Square = Square::A8;
pub const B8: Square = Square::B8;
pub const C8: Square = Square::C8;
pub const D8: Square = Square::D8;
pub const E8: Square = Square::E8;
pub const F8: Square = Square::F8;
pub const G8: Square = Square::G8;
pub const H8: Square = Square::H8;

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.file_char(), self.rank_char())
    }
}

impl FromStr for Square {
    type Err = FenError;

    fn from_str(s: &str) -> FenResult<Self> {
        if s.len() != 2 {
            return Err(FenError::InvalidSquare(s.to_string()));
        }

        let file_char = s.chars().nth(0).unwrap().to_ascii_uppercase();
        let rank_char = s.chars().nth(1).unwrap();

        let file: u8 = match file_char {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => {
                return Err(FenError::InvalidSquare(format!(
                    "Invalid file '{file_char}'"
                )));
            }
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
            _ => {
                return Err(FenError::InvalidSquare(format!(
                    "Invalid rank '{rank_char}'"
                )));
            }
        };

        Ok(Square::from_file_rank(file, rank))
    }
}
