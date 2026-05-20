use crate::core::square::Square;
use crate::core::u16_get_bit;
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
/// An u64 where every bit represents one cell of the chess board.
///
/// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
/// ```text
///     A    B    C    D    E    F    G    H
///   +----+----+----+----+----+----+----+----+
/// 8 | 56 | 57 | 58 | 59 | 60 | 61 | 62 | 63 |
///   +----+----+----+----+----+----+----+----+
/// 7 | 48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 |
///   +----+----+----+----+----+----+----+----+
/// 6 | 40 | 41 | 42 | 43 | 44 | 45 | 46 | 47 |
///   +----+----+----+----+----+----+----+----+
/// 5 | 32 | 33 | 34 | 35 | 36 | 37 | 38 | 39 |
///   +----+----+----+----+----+----+----+----+
/// 4 | 24 | 25 | 26 | 27 | 28 | 29 | 30 | 31 |
///   +----+----+----+----+----+----+----+----+
/// 3 | 16 | 17 | 18 | 19 | 20 | 21 | 22 | 23 |
///   +----+----+----+----+----+----+----+----+
/// 2 |  8 |  9 | 10 | 11 | 12 | 13 | 14 | 15 |
///   +----+----+----+----+----+----+----+----+
/// 1 |  0 |  1 |  2 |  3 |  4 |  5 |  6 |  7 |
///   +----+----+----+----+----+----+----+----+
/// ```
pub struct BitBoard(u64);

impl BitBoard {
    /// Create a new [`BitBoard`] from a u64.
    ///
    /// # Arguments
    ///
    /// * `value`: The bitboard value.
    ///
    /// returns: [`BitBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    ///
    /// let value = 0b1100110;
    /// let bb = BitBoard::new(value);
    ///
    /// assert_eq!(bb.value(), value);
    /// ```
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Creates a new [`BitBoard`] where no bits are set.
    ///
    /// Returns: [`BitBoard`]
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    ///
    /// let bb = BitBoard::empty();
    ///
    /// assert_eq!(bb.value(), 0);
    /// ```
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the raw value of the [`BitBoard`].
    ///
    /// Returns: u64
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    ///
    /// let value = 0b1100110;
    /// let bb = BitBoard::new(value);
    ///
    /// assert_eq!(bb.value(), value);
    /// ```
    pub fn value(&self) -> u64 {
        self.0
    }

    /// Returns true if no bits are set, else false.
    ///
    /// Returns: bool
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    ///
    /// let not_empty = BitBoard::new(0b11001);
    /// let empty = BitBoard::new(0);
    ///
    /// assert!(!not_empty.is_empty());
    /// assert!(empty.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if the bit at the given index is set, else false.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// # Arguments
    /// * `square`: The square to check.
    ///
    /// Returns: bool
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::core::square::Square;
    ///
    /// let bb = BitBoard::new(0b11001);
    ///
    /// assert!(bb.is_set(Square::A1));
    /// assert!(!bb.is_set(Square::B1));
    /// assert!(!bb.is_set(Square::C1));
    /// assert!(bb.is_set(Square::D1));
    /// assert!(bb.is_set(Square::E1));
    /// assert!(!bb.is_set(Square::F1));
    /// ```
    pub fn is_set(&self, square: Square) -> bool {
        (self.0 & (1u64 << square.index())) != 0u64
    }

    /// Sets the bit at the given index to 1.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// # Arguments
    /// * `square`: The square to set.
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::core::square::Square;
    ///
    /// let mut bb = BitBoard::empty();
    /// bb.set(Square::A1);
    /// bb.set(Square::C1);
    /// bb.set(Square::F1);
    ///
    /// assert_eq!(bb.value(), 0b100101);
    /// ```
    pub fn set(&mut self, square: Square) {
        self.0 |= 1u64 << square.index();
    }

    /// Sets the bit at the given index to 0.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// # Arguments
    /// * `square`: The square to clear.
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::core::square::Square;
    ///
    /// let mut bb = BitBoard::new(0b100101);
    /// bb.clear(Square::A1);
    /// bb.clear(Square::C1);
    /// bb.clear(Square::F1);
    ///
    /// assert_eq!(bb.value(), 0);
    /// ```
    pub fn clear(&mut self, square: Square) {
        self.0 &= !(1u64 << square.index());
    }

    /// Counts the number of set bits.
    ///
    /// Returns: u8
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    ///
    /// let bb = BitBoard::new(0b100101);
    ///
    /// assert_eq!(bb.count_set(), 3);
    /// ```
    pub fn count_set(&self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Retrieves the index of the lowest set bit if there is one set bit, else None.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// Returns: Option<Square>
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::core::square::Square;
    ///
    /// let bb_1 = BitBoard::new(0b100100);
    /// assert_eq!(bb_1.get_lowest_set(), Some(Square::C1));
    ///
    /// let bb_2 = BitBoard::empty();
    /// assert_eq!(bb_2.get_lowest_set(), None);
    /// ```
    pub fn get_lowest_set(&self) -> Option<Square> {
        if self.0 == 0 {
            return None;
        }
        Some(Square::new(self.0.trailing_zeros() as u8))
    }

    /// Clears the lowest set bit and returns its index if there is one set bit, else None.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// Returns: Option<Square>
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::core::square::Square;
    ///
    /// let mut bb = BitBoard::new(0b100100);
    ///
    /// assert_eq!(bb.pop_lowest_set(), Some(Square::C1));
    /// assert_eq!(bb.pop_lowest_set(), Some(Square::F1));
    /// assert_eq!(bb.pop_lowest_set(), None);
    /// ```
    pub fn pop_lowest_set(&mut self) -> Option<Square> {
        let lsb = self.get_lowest_set()?;
        self.0 &= self.0 - 1;
        Some(lsb)
    }

    /// Iterates over all set squares of the board.
    ///
    /// Returns: [`BitBoardIter`]
    ///
    /// # Examples
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::core::square::Square;
    ///
    /// let mut bb = BitBoard::new(0b100101);
    ///
    /// let mut squares = Vec::new();
    /// for square in bb.iter() {
    ///     squares.push(square);
    /// }
    ///
    /// assert_eq!(squares, vec![Square::A1, Square::C1, Square::F1]);
    /// ```
    pub fn iter(&self) -> BitBoardIter {
        BitBoardIter { bits: self.0 }
    }

    pub fn occupancy_variation(&self, index: u16) -> BitBoard {
        let mut mask = *self;
        let mut result = BitBoard::empty();
        for variation_index in 0u8..16 {
            if let Some(lowest_set_bit_index) = mask.pop_lowest_set() {
                if u16_get_bit(index, variation_index) {
                    result.set(lowest_set_bit_index);
                }
            } else {
                break;
            }
        }
        result
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard::new(self.0 | rhs.0)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        BitBoard::new(self.0 & rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = Self;

    fn bitxor(self, rhs: Self) -> Self::Output {
        BitBoard::new(self.0 ^ rhs.0)
    }
}

impl Not for BitBoard {
    type Output = Self;

    fn not(self) -> Self::Output {
        BitBoard::new(!self.0)
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl From<BitBoard> for u64 {
    fn from(value: BitBoard) -> Self {
        value.value()
    }
}

impl From<u64> for BitBoard {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}

/// An iterator over all set squares of a [`BitBoard`].
pub struct BitBoardIter {
    bits: u64,
}

impl Iterator for BitBoardIter {
    type Item = Square;

    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }
        let index = self.bits.trailing_zeros() as u8;
        self.bits &= self.bits - 1;
        Some(Square::new(index))
    }
}

impl IntoIterator for BitBoard {
    type Item = Square;
    type IntoIter = BitBoardIter;
    fn into_iter(self) -> Self::IntoIter {
        BitBoardIter { bits: self.0 }
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for square in Square::iter_top_bottom() {
            if self.is_set(square) {
                write!(f, "1 ")?;
            } else {
                write!(f, "0 ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn test_or() {
        let b1 = BitBoard::new(0b0101);
        let b2 = BitBoard::new(0b1010);
        assert_eq!((b1 | b2).value(), 0b1111)
    }

    #[test]
    fn test_and() {
        let b1 = BitBoard::new(0b0101);
        let b2 = BitBoard::new(0b1110);
        assert_eq!((b1 & b2).value(), 0b0100)
    }

    #[test]
    fn test_xor() {
        let b1 = BitBoard::new(0b0101);
        let b2 = BitBoard::new(0b1110);
        assert_eq!((b1 ^ b2).value(), 0b1011)
    }

    #[test]
    fn test_not() {
        let bb = BitBoard::new(0xF);
        assert_eq!((!bb).value(), 0xFFFFFFFFFFFFFFF0)
    }
}
