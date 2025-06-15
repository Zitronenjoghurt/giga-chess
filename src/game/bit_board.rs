use crate::utils::bit_operations::u16_get_bit;
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
/// A u64 where every bit represents one cell of the chess board.
///
/// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
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
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let value = 0b1100110;
    /// let bb = BitBoard::new(value);
    ///
    /// assert_eq!(bb.get_value(), value);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    /// Creates a new [`BitBoard`] where no bits are set.
    ///
    /// returns: [`BitBoard`]
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let bb = BitBoard::empty();
    ///
    /// assert_eq!(bb.get_value(), 0);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns the raw value of the [`BitBoard`].
    ///
    /// returns: u64
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let value = 0b1100110;
    /// let bb = BitBoard::new(value);
    ///
    /// assert_eq!(bb.get_value(), value);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn get_value(&self) -> u64 {
        self.0
    }

    /// Returns true if no bits are set, else false.
    ///
    /// returns: bool
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let not_empty = BitBoard::new(0b11001);
    /// let empty = BitBoard::new(0);
    ///
    /// assert!(!not_empty.is_empty());
    /// assert!(empty.is_empty());
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    /// Returns true if the bit at the given index is set, else false.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// # Arguments
    /// * `index`: The index of the bit to check.
    ///
    /// returns: bool
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let bb = BitBoard::new(0b11001);
    ///
    /// assert!(bb.get_bit(0));
    /// assert!(!bb.get_bit(1));
    /// assert!(!bb.get_bit(2));
    /// assert!(bb.get_bit(3));
    /// assert!(bb.get_bit(4));
    /// assert!(!bb.get_bit(5));
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn get_bit(&self, index: u8) -> bool {
        (self.0 & (1u64 << index)) != 0u64
    }

    /// Sets the bit at the given index to 1.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// # Arguments
    /// * `index`: The index of the bit to set.
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let mut bb = BitBoard::empty();
    /// bb.set_bit(0);
    /// bb.set_bit(2);
    /// bb.set_bit(5);
    ///
    /// assert_eq!(bb.get_value(), 0b100101);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1u64 << index;
    }

    /// Sets the bit at the given index to 0.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// # Arguments
    /// * `index`: The index of the bit to clear.
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let mut bb = BitBoard::new(0b100101);
    /// bb.clear_bit(0);
    /// bb.clear_bit(2);
    /// bb.clear_bit(5);
    ///
    /// assert_eq!(bb.get_value(), 0);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1u64 << index);
    }

    /// Counts the number of set bits.
    ///
    /// returns: u8
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let bb = BitBoard::new(0b100101);
    ///
    /// assert_eq!(bb.count_set_bits(), 3);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn count_set_bits(&self) -> u8 {
        self.0.count_ones() as u8
    }

    /// Retrieves the index of the lowest set bit if there is one set bit, else None.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// returns: Option<u8>
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let bb_1 = BitBoard::new(0b100100);
    /// assert_eq!(bb_1.get_lowest_set_bit(), Some(2));
    ///
    /// let bb_2 = BitBoard::empty();
    /// assert_eq!(bb_2.get_lowest_set_bit(), None);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn get_lowest_set_bit(&self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }
        Some(self.0.trailing_zeros() as u8)
    }

    /// Clears the lowest set bit and returns its index if there is one set bit, else None.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// returns: Option<u8>
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let mut bb = BitBoard::new(0b100100);
    ///
    /// assert_eq!(bb.pop_lowest_set_bit(), Some(2));
    /// assert_eq!(bb.pop_lowest_set_bit(), Some(5));
    /// assert_eq!(bb.pop_lowest_set_bit(), None);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn pop_lowest_set_bit(&mut self) -> Option<u8> {
        let lsb_index = self.get_lowest_set_bit()?;
        self.0 &= self.0 - 1;
        Some(lsb_index)
    }

    /// Iterates over the indices of all set bits.
    ///
    /// Indexing starts with the least significant bit (0) and ends with the most significant bit (63).
    ///
    /// returns: [`BitBoardIter`]
    ///
    /// # Examples
    /// ```
    /// use giga_chess::game::bit_board::BitBoard;
    ///
    /// let mut bb = BitBoard::new(0b100101);
    ///
    /// let mut indices = Vec::new();
    /// for index in bb.iter_set_bits() {
    ///     indices.push(index);
    /// }
    ///
    /// assert_eq!(indices, vec![0, 2, 5]);
    /// ```
    #[cfg_attr(tarpaulin, inline(never))]
    #[cfg_attr(not(tarpaulin), inline(always))]
    pub fn iter_set_bits(&self) -> BitBoardIter {
        BitBoardIter { bits: self.0 }
    }

    pub fn occupancy_variation(&self, index: u16) -> BitBoard {
        let mut mask = *self;
        let mut result = BitBoard::empty();
        for variation_index in 0u8..16 {
            if let Some(lowest_set_bit_index) = mask.pop_lowest_set_bit() {
                if u16_get_bit(index, variation_index) {
                    result.set_bit(lowest_set_bit_index);
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

/// An iterator over the indices of all set bits of a [`BitBoard`].
pub struct BitBoardIter {
    bits: u64,
}

impl Iterator for BitBoardIter {
    type Item = u8;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.bits == 0 {
            return None;
        }

        let index = self.bits.trailing_zeros() as u8;
        self.bits &= self.bits - 1;
        Some(index)
    }
}

impl Display for BitBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in (0..8).rev() {
            for x in 0..8 {
                let index: u8 = x + y * 8;
                if self.get_bit(index) {
                    write!(f, "1 ")?;
                } else {
                    write!(f, "0 ")?;
                }
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
        assert_eq!((b1 | b2).get_value(), 0b1111)
    }

    #[test]
    fn test_and() {
        let b1 = BitBoard::new(0b0101);
        let b2 = BitBoard::new(0b1110);
        assert_eq!((b1 & b2).get_value(), 0b0100)
    }

    #[test]
    fn test_xor() {
        let b1 = BitBoard::new(0b0101);
        let b2 = BitBoard::new(0b1110);
        assert_eq!((b1 ^ b2).get_value(), 0b1011)
    }

    #[test]
    fn test_not() {
        let bb = BitBoard::new(0xF);
        assert_eq!((!bb).get_value(), 0xFFFFFFFFFFFFFFF0)
    }
}
