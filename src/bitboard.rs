/// A u64 where every bit represents every cell of the chess board
/// For performance reasons it is inferred that all indices operating on this board are between 0-63
pub struct BitBoard(pub u64);

impl BitBoard {
    #[inline]
    /// Returns true if the bit at the given index is 1
    pub fn get_bit(&self, index: u8) -> bool {
        (self.0 & (1 << index)) != 0
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
}
