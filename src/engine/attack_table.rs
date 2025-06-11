use crate::engine::bit_board::BitBoard;
use crate::engine::square::Square;
use crate::game::color::Color;

pub struct AttackTable {
    pub pawn_attacks: [[BitBoard; 64]; 2],
    pub knight_attacks: [BitBoard; 64],
    pub king_attacks: [BitBoard; 64],
    pub bishop_masks: [BitBoard; 64],
    pub rook_masks: [BitBoard; 64],
}

impl AttackTable {
    pub fn build() -> Self {
        Self {
            pawn_attacks: build_pawn_attacks(),
            knight_attacks: build_knight_attacks(),
            king_attacks: build_king_attacks(),
            bishop_masks: build_bishop_masks(),
            rook_masks: build_rook_masks(),
        }
    }
}

fn build_pawn_attacks() -> [[BitBoard; 64]; 2] {
    let mut table = [[BitBoard::empty(); 64]; 2];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut white_attacks = 0u64;
        if let Some(index) = square.index_up_left() {
            white_attacks |= 1 << index;
        }
        if let Some(index) = square.index_up_right() {
            white_attacks |= 1 << index;
        }
        table[Color::White as usize][index] = BitBoard::new(white_attacks);

        let mut black_attacks = 0u64;
        if let Some(index) = square.index_down_left() {
            black_attacks |= 1 << index;
        }
        if let Some(index) = square.index_down_right() {
            black_attacks |= 1 << index;
        }
        table[Color::Black as usize][index] = BitBoard::new(black_attacks);
    }

    table
}

fn build_knight_attacks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut attacks = 0u64;
        if let Some(index) = square.index_jump(1, 2) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(2, 1) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(2, -1) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(1, -2) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(-1, -2) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(-2, -1) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(-2, 1) {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_jump(-1, 2) {
            attacks |= 1 << index;
        }

        table[index] = BitBoard::new(attacks);
    }

    table
}

fn build_king_attacks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut attacks = 0u64;
        if let Some(index) = square.index_up() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_down() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_left() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_right() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_up_left() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_up_right() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_down_left() {
            attacks |= 1 << index;
        }
        if let Some(index) = square.index_down_right() {
            attacks |= 1 << index;
        }

        table[index] = BitBoard::new(attacks);
    }

    table
}

fn build_bishop_masks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut mask = 0u64;
        for index in square.trace_up_left() {
            mask |= 1 << index;
        }
        for index in square.trace_up_right() {
            mask |= 1 << index;
        }
        for index in square.trace_down_left() {
            mask |= 1 << index;
        }
        for index in square.trace_down_right() {
            mask |= 1 << index;
        }

        table[index] = BitBoard::new(mask);
    }

    table
}

fn build_rook_masks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut mask = 0u64;
        for index in square.trace_up() {
            mask |= 1 << index;
        }
        for index in square.trace_down() {
            mask |= 1 << index;
        }
        for index in square.trace_left() {
            mask |= 1 << index;
        }
        for index in square.trace_right() {
            mask |= 1 << index;
        }

        table[index] = BitBoard::new(mask);
    }

    table
}
