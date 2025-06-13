use crate::utils::bit_operations::u16_get_bit;
use std::fmt::{Display, Formatter};
use std::ops::{BitAnd, BitOr};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[repr(transparent)]
/// A u64 where every bit represents one cell of the chess board
pub struct BitBoard(u64);

impl BitBoard {
    #[inline(always)]
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    #[inline(always)]
    pub const fn empty() -> Self {
        Self(0)
    }

    #[inline(always)]
    pub fn get_value(&self) -> u64 {
        self.0
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    /// Returns true if the bit at the given index is 1
    pub fn get_bit(&self, index: u8) -> bool {
        (self.0 & (1u64 << index)) != 0u64
    }

    #[inline(always)]
    /// Sets the bit at index to 1
    pub fn set_bit(&mut self, index: u8) {
        self.0 |= 1u64 << index;
    }

    #[inline(always)]
    /// Sets the bit at index to 0
    pub fn clear_bit(&mut self, index: u8) {
        self.0 &= !(1u64 << index);
    }

    #[inline(always)]
    pub fn count_ones(&self) -> u8 {
        self.0.count_ones() as u8
    }

    #[inline(always)]
    pub fn pop_lowest_set_bit(&mut self) -> Option<u8> {
        if self.0 == 0 {
            return None;
        }

        let lsb_index = self.0.trailing_zeros() as u8;
        self.0 &= self.0 - 1;
        Some(lsb_index)
    }

    pub fn occupancy_variation(mut mask: BitBoard, index: u16) -> BitBoard {
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
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn test_get_bit() {
        let b = BitBoard::new(0b0101);
        assert!(b.get_bit(0));
        assert!(!b.get_bit(1));
        assert!(b.get_bit(2));
        assert!(!b.get_bit(3));
    }

    #[test]
    fn test_set_bit() {
        let mut b = BitBoard::empty();
        b.set_bit(0);
        b.set_bit(1);
        b.set_bit(2);
        b.set_bit(3);
        assert_eq!(b.get_value(), 0b1111)
    }

    #[test]
    fn test_clear_bit() {
        let mut b = BitBoard::new(0b1111);
        b.clear_bit(0);
        b.clear_bit(1);
        b.clear_bit(2);
        b.clear_bit(3);
        assert_eq!(b.get_value(), 0)
    }

    #[test]
    fn test_or() {
        let b1 = BitBoard::new(0b0101);
        let b2 = BitBoard::new(0b1010);
        assert_eq!((b1 | b2).get_value(), 0b1111)
    }
}
