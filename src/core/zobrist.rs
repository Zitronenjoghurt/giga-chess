use crate::core::position::Position;
use crate::prelude::{CastlingRights, Color, Piece, Square};
use std::sync::OnceLock;

const NUM_PIECES: usize = 6;
const NUM_COLORS: usize = 2;
const NUM_SQUARES: usize = 64;
const NUM_CASTLING: usize = 16; // 4 bits
const NUM_EP_FILES: usize = 8;

static KEYS: OnceLock<ZobristKeys> = OnceLock::new();

pub struct ZobristKeys {
    pieces: [[[u64; NUM_SQUARES]; NUM_COLORS]; NUM_PIECES],
    side_to_move: u64,
    castling: [u64; NUM_CASTLING],
    en_passant: [u64; NUM_EP_FILES],
}

impl ZobristKeys {
    pub fn get() -> &'static ZobristKeys {
        KEYS.get_or_init(Self::generate)
    }

    fn generate() -> Self {
        let mut rng = Xorshift64(0x98f107a3bc8d61e5);
        Self {
            pieces: std::array::from_fn(|_| {
                std::array::from_fn(|_| std::array::from_fn(|_| rng.next()))
            }),
            side_to_move: rng.next(),
            castling: std::array::from_fn(|_| rng.next()),
            en_passant: std::array::from_fn(|_| rng.next()),
        }
    }

    pub fn full_hash(pos: &Position) -> u64 {
        let keys = Self::get();
        let mut hash = 0u64;

        for sq in Square::iter_bottom_top() {
            if let Some((piece, color)) = pos.board.piece_at(sq) {
                hash ^= keys.pieces[piece as usize][color as usize][sq.index() as usize];
            }
        }

        if pos.side_to_move == Color::Black {
            hash ^= keys.side_to_move;
        }

        hash ^= keys.castling[pos.castling_rights.bits() as usize];

        if let Some(ep_sq) = pos.en_passant_square {
            hash ^= keys.en_passant[(ep_sq.file() - 1) as usize];
        }

        hash
    }

    pub fn piece_key(piece: Piece, color: Color, sq: Square) -> u64 {
        Self::get().pieces[piece as usize][color as usize][sq.index() as usize]
    }

    pub fn side_key() -> u64 {
        Self::get().side_to_move
    }

    pub fn castling_key(rights: &CastlingRights) -> u64 {
        Self::get().castling[rights.bits() as usize]
    }

    pub fn ep_key(sq: Square) -> u64 {
        Self::get().en_passant[(sq.file() - 1) as usize]
    }
}

struct Xorshift64(u64);
impl Xorshift64 {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
}
