pub mod bitboard;
pub mod board;
pub mod castling;
pub mod piece;
pub mod position;
pub mod square;
pub mod zobrist;

fn u16_get_bit(value: u16, index: u8) -> bool {
    (value & (1 << index)) != 0
}
