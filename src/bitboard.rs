/// A u64 where every bit represents every cell of the chess board
pub struct BitBoard(pub u64);

impl BitBoard {
    #[inline]
    /// Returns true if the bit at the given index is 1
    pub fn get_bit(&self, index: u8) -> bool {
        if index > 63 {
            return false;
        }
        (self.0 & (1u64 << index)) != 0u64
    }

    #[inline]
    /// Sets the bit at index to 1
    pub fn set_bit(&mut self, index: u8) {
        if index > 63 {
            return;
        }
        self.0 |= 1u64 << index;
    }

    #[inline]
    /// Sets the bit at index to 0
    pub fn clear_bit(&mut self, index: u8) {
        if index > 63 {
            return;
        }
        self.0 &= !(1u64 << index);
    }
}

#[cfg(test)]
mod tests {
    use super::BitBoard;

    #[test]
    fn test_get_bit() {
        let b = BitBoard(0b0101);
        assert!(b.get_bit(0));
        assert!(!b.get_bit(1));
        assert!(b.get_bit(2));
        assert!(!b.get_bit(3));
    }

    #[test]
    fn test_set_bit() {
        let mut b = BitBoard(0);
        b.set_bit(0);
        b.set_bit(1);
        b.set_bit(2);
        b.set_bit(3);
        assert_eq!(b.0, 0b1111)
    }

    #[test]
    fn test_clear_bit() {
        let mut b = BitBoard(0b1111);
        b.clear_bit(0);
        b.clear_bit(1);
        b.clear_bit(2);
        b.clear_bit(3);
        assert_eq!(b.0, 0)
    }
}
