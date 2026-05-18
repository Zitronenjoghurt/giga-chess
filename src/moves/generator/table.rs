use crate::core::bitboard::BitBoard;
use crate::moves::generator::magic::magic_hash;
use crate::prelude::{Color, Square};

const BISHOP_MAGICS: [u64; 64] = [
    0x0106004105020080,
    0x0010002A00081020,
    0x2800208400404020,
    0x0148400441014000,
    0x0420221208840110,
    0x02204C204103B408,
    0x1080540200600884,
    0x8501021404100400,
    0x0025228110004010,
    0x0200D48002044048,
    0x0004004200810004,
    0x8042088040800008,
    0x0010040020000000,
    0x0000428244208800,
    0x0080222404000811,
    0x80082011A0100068,
    0x0120000982004208,
    0x0246A60100400CD0,
    0x2411400A01020008,
    0x04804802000B0000,
    0x0000080080844000,
    0x00811220010C0202,
    0x002200080C010811,
    0x0000004022081A00,
    0x0800044841803050,
    0x200402088400800B,
    0x4002020801800126,
    0x0200081000820500,
    0x40028C000820D022,
    0x00A2500418407800,
    0x0000114000080110,
    0x040901C002043004,
    0x8026118400001490,
    0x11A1100080021004,
    0x00020042C2004010,
    0x8204300800440501,
    0x18040AC004018080,
    0x6182801310025200,
    0x000021004880900C,
    0x1010210007120010,
    0xA405840101005800,
    0x2950118025200442,
    0x000684E218000020,
    0x0004502020010022,
    0x0041AC001308040C,
    0x000103080A000824,
    0x0020100200800520,
    0x2008804446040802,
    0x0011219800088001,
    0x0000408010814088,
    0x08830B4040202040,
    0x0004325080400401,
    0x0000100020240008,
    0x0000019200250000,
    0x0604078200881040,
    0x4028180444120000,
    0x8000010128088C70,
    0x0360004004811050,
    0x00B5280044100C04,
    0x00020010020C2001,
    0x0100200128010819,
    0x4803120C41100090,
    0x0042004192008008,
    0x0006861020210069,
];

const ROOK_MAGICS: [u64; 64] = [
    0x0080002040001084,
    0x00092030804082C0,
    0x0020040008202002,
    0x0020040820200200,
    0x0100100408400A23,
    0x0020088C00030200,
    0x4288008003000200,
    0x0080012100044080,
    0x1090280040008000,
    0x0500060840242004,
    0x14004A0220002800,
    0x2100A00204401000,
    0x0211201101200426,
    0x40008022800CDC00,
    0x4204015002088401,
    0x0065000100012E42,
    0x0080100804004140,
    0x2860040804008022,
    0x0A20300450061100,
    0x0002002080140420,
    0x1210282802010514,
    0x1220048000C10142,
    0x0020202002508A00,
    0x0C42208021904101,
    0x1080400080056082,
    0x0188000804210108,
    0x0010200040040801,
    0x86000A0C10003912,
    0x0000011120040A08,
    0x0000009A02005280,
    0x1000040488080940,
    0x4000041002006008,
    0x0008001048022424,
    0x0000A60426004000,
    0x00A8600808010260,
    0x2090015080080300,
    0x0004020100080C60,
    0x002004022D004010,
    0x8003000100400082,
    0x0A00100020100940,
    0x000400C006100800,
    0x001AA00422122560,
    0x602020104C015000,
    0x0844180802001000,
    0x1044BC0100202002,
    0x0C01280402810002,
    0xC284011031D04200,
    0x1210110143200406,
    0x1209012580413100,
    0x8780300104201004,
    0xAA002806020E0008,
    0x0001900080023002,
    0x0808050018002424,
    0x0001042401088020,
    0x0040020030E30080,
    0x000001140880A200,
    0x000107A040800015,
    0x0040800820110043,
    0x001028800A00C222,
    0x0002000410480802,
    0x0148704800040213,
    0x1800040100008801,
    0x0064063008058114,
    0x18400034840101C2,
];

#[derive(Debug)]
pub struct AttackTable {
    pawn_masks: [[BitBoard; 64]; 2],
    pawn_attacks: [[BitBoard; 64]; 2],
    knight_attacks: [BitBoard; 64],
    king_attacks: [BitBoard; 64],
    bishop_masks: [BitBoard; 64],
    rook_masks: [BitBoard; 64],
    bishop_blocks: [BitBoard; 64],
    rook_blocks: [BitBoard; 64],
    bishop_attacks: Vec<Vec<BitBoard>>,
    rook_attacks: Vec<Vec<BitBoard>>,
}

impl Default for AttackTable {
    fn default() -> Self {
        Self::build()
    }
}

impl AttackTable {
    pub fn build() -> Self {
        Self {
            pawn_masks: build_pawn_masks(),
            pawn_attacks: build_pawn_attacks(),
            knight_attacks: build_knight_attacks(),
            king_attacks: build_king_attacks(),
            bishop_masks: build_bishop_masks(),
            rook_masks: build_rook_masks(),
            bishop_blocks: build_bishop_blocks(),
            rook_blocks: build_rook_blocks(),
            bishop_attacks: build_bishop_attacks(),
            rook_attacks: build_rook_attacks(),
        }
    }

    pub fn pawn_mask(&self, square: Square, color: Color) -> BitBoard {
        self.pawn_masks[color as usize][square.index() as usize]
    }

    pub fn pawn_attacks(&self, square: Square, color: Color) -> BitBoard {
        self.pawn_attacks[color as usize][square.index() as usize]
    }

    pub fn pawn_king_attack(&self, square: Square, color: Color) -> BitBoard {
        self.pawn_attacks[color.opposite() as usize][square.index() as usize]
    }

    pub fn knight_attacks(&self, square: Square) -> BitBoard {
        self.knight_attacks[square.index() as usize]
    }

    pub fn king_attacks(&self, square: Square) -> BitBoard {
        self.king_attacks[square.index() as usize]
    }

    pub fn bishop_mask(&self, square: Square) -> BitBoard {
        self.bishop_masks[square.index() as usize]
    }

    pub fn rook_mask(&self, square: Square) -> BitBoard {
        self.rook_masks[square.index() as usize]
    }

    pub fn queen_mask(&self, square: Square) -> BitBoard {
        self.bishop_mask(square) | self.rook_mask(square)
    }

    pub fn bishop_blocks(&self, square: Square) -> BitBoard {
        self.bishop_blocks[square.index() as usize]
    }

    pub fn rook_blocks(&self, square: Square) -> BitBoard {
        self.rook_blocks[square.index() as usize]
    }

    pub fn queen_blocks(&self, square: Square) -> BitBoard {
        self.bishop_blocks(square) | self.rook_blocks(square)
    }

    pub fn bishop_attacks(&self, square: Square, occupancy: BitBoard) -> BitBoard {
        let block_mask = self.bishop_blocks(square);
        let occupied_blocking_mask = occupancy & block_mask;
        let key = magic_hash(
            BISHOP_MAGICS[square.index() as usize],
            occupied_blocking_mask,
        );
        self.bishop_attacks[square.index() as usize][key]
    }

    pub fn rook_attacks(&self, square: Square, occupancy: BitBoard) -> BitBoard {
        let block_mask = self.rook_blocks(square);
        let occupied_blocking_mask = occupancy & block_mask;
        let key = magic_hash(ROOK_MAGICS[square.index() as usize], occupied_blocking_mask);
        self.rook_attacks[square.index() as usize][key]
    }

    pub fn queen_attacks(&self, square: Square, occupancy: BitBoard) -> BitBoard {
        self.bishop_attacks(square, occupancy) | self.rook_attacks(square, occupancy)
    }
}

pub fn build_pawn_masks() -> [[BitBoard; 64]; 2] {
    let mut table = [[BitBoard::empty(); 64]; 2];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut white_mask = 0u64;
        if let Some(sq) = square.up() {
            white_mask |= 1 << sq.index();
        }
        table[Color::White as usize][index] = BitBoard::new(white_mask);

        let mut black_mask = 0u64;
        if let Some(sq) = square.down() {
            black_mask |= 1 << sq.index();
        }
        table[Color::Black as usize][index] = BitBoard::new(black_mask);
    }

    table
}

pub fn build_pawn_attacks() -> [[BitBoard; 64]; 2] {
    let mut table = [[BitBoard::empty(); 64]; 2];

    for index in 0usize..64 {
        let square = Square::new(index as u8);

        let mut white_attacks = 0u64;
        if let Some(sq) = square.up_left() {
            white_attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.up_right() {
            white_attacks |= 1 << sq.index();
        }
        table[Color::White as usize][index] = BitBoard::new(white_attacks);

        let mut black_attacks = 0u64;
        if let Some(sq) = square.down_left() {
            black_attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.down_right() {
            black_attacks |= 1 << sq.index();
        }
        table[Color::Black as usize][index] = BitBoard::new(black_attacks);
    }

    table
}

pub fn build_knight_attacks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for (index, bb) in table.iter_mut().enumerate() {
        let square = Square::new(index as u8);

        let mut attacks = 0u64;
        if let Some(sq) = square.jump(1, 2) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(2, 1) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(2, -1) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(1, -2) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(-1, -2) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(-2, -1) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(-2, 1) {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.jump(-1, 2) {
            attacks |= 1 << sq.index();
        }

        *bb = BitBoard::new(attacks);
    }

    table
}

pub fn build_king_attacks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for (index, bb) in table.iter_mut().enumerate() {
        let square = Square::new(index as u8);

        let mut attacks = 0u64;
        if let Some(sq) = square.up() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.down() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.left() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.right() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.up_left() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.up_right() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.down_left() {
            attacks |= 1 << sq.index();
        }
        if let Some(sq) = square.down_right() {
            attacks |= 1 << sq.index();
        }

        *bb = BitBoard::new(attacks);
    }

    table
}

pub fn build_bishop_masks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for (index, bb) in table.iter_mut().enumerate() {
        let square = Square::new(index as u8);

        let mut mask = 0u64;
        for sq in square.trace_up_left() {
            mask |= 1 << sq.index();
        }
        for sq in square.trace_up_right() {
            mask |= 1 << sq.index();
        }
        for sq in square.trace_down_left() {
            mask |= 1 << sq.index();
        }
        for sq in square.trace_down_right() {
            mask |= 1 << sq.index();
        }

        *bb = BitBoard::new(mask);
    }

    table
}

pub fn build_rook_masks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for (index, bb) in table.iter_mut().enumerate() {
        let square = Square::new(index as u8);

        let mut mask = 0u64;
        for sq in square.trace_up() {
            mask |= 1 << sq.index();
        }
        for sq in square.trace_down() {
            mask |= 1 << sq.index();
        }
        for sq in square.trace_left() {
            mask |= 1 << sq.index();
        }
        for sq in square.trace_right() {
            mask |= 1 << sq.index();
        }

        *bb = BitBoard::new(mask);
    }

    table
}

pub fn build_bishop_blocks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for (index, bb) in table.iter_mut().enumerate() {
        let square = Square::new(index as u8);

        let mut mask = 0u64;

        let up_left: Vec<Square> = square.trace_up_left().collect();
        for &sq in &up_left[..up_left.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        let up_right: Vec<Square> = square.trace_up_right().collect();
        for &sq in &up_right[..up_right.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        let down_left: Vec<Square> = square.trace_down_left().collect();
        for &sq in &down_left[..down_left.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        let down_right: Vec<Square> = square.trace_down_right().collect();
        for &sq in &down_right[..down_right.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        *bb = BitBoard::new(mask);
    }

    table
}

pub fn build_rook_blocks() -> [BitBoard; 64] {
    let mut table = [BitBoard::empty(); 64];

    for (index, bb) in table.iter_mut().enumerate() {
        let square = Square::new(index as u8);

        let mut mask = 0u64;

        let up: Vec<Square> = square.trace_up().collect();
        for &sq in &up[..up.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        let down: Vec<Square> = square.trace_down().collect();
        for &sq in &down[..down.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        let left: Vec<Square> = square.trace_left().collect();
        for &sq in &left[..left.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        let right: Vec<Square> = square.trace_right().collect();
        for &sq in &right[..right.len().saturating_sub(1)] {
            mask |= 1 << sq.index();
        }

        *bb = BitBoard::new(mask);
    }

    table
}

pub fn calculate_bishop_attack(square: u8, occupancy: BitBoard) -> BitBoard {
    let mut result = BitBoard::empty();
    let square = Square::new(square);

    for index in square.trace_up_left() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    for index in square.trace_up_right() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    for index in square.trace_down_left() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    for index in square.trace_down_right() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    result
}

pub fn calculate_rook_attack(square: u8, occupancy: BitBoard) -> BitBoard {
    let mut result = BitBoard::empty();
    let square = Square::new(square);

    for index in square.trace_up() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    for index in square.trace_down() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    for index in square.trace_left() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    for index in square.trace_right() {
        result.set(index);
        if occupancy.is_set(index) {
            break;
        }
    }

    result
}

pub fn build_occupancy_variations(block_mask: BitBoard) -> Vec<BitBoard> {
    let occupancy_count = 1usize << block_mask.count_set();
    (0..occupancy_count)
        .map(|index| block_mask.occupancy_variation(index as u16))
        .collect()
}

pub fn build_bishop_attacks() -> Vec<Vec<BitBoard>> {
    let mut table = Vec::new();
    let block_masks = build_bishop_blocks();

    for square in 0..64 {
        let magic = BISHOP_MAGICS[square];
        let block_mask = block_masks[square];
        let occupancies = build_occupancy_variations(block_mask);
        let attacks: Vec<BitBoard> = occupancies
            .iter()
            .map(|occupancy| calculate_bishop_attack(square as u8, *occupancy))
            .collect();

        let mut square_table = vec![BitBoard::empty(); 4096];
        for i in 0usize..occupancies.len() {
            let key = magic_hash(magic, occupancies[i]);
            square_table[key] = attacks[i];
        }
        table.push(square_table);
    }

    table
}

pub fn build_rook_attacks() -> Vec<Vec<BitBoard>> {
    let mut table = Vec::new();
    let block_masks = build_rook_blocks();

    for square in 0..64 {
        let magic = ROOK_MAGICS[square];
        let block_mask = block_masks[square];
        let occupancies = build_occupancy_variations(block_mask);
        let attacks: Vec<BitBoard> = occupancies
            .iter()
            .map(|occupancy| calculate_rook_attack(square as u8, *occupancy))
            .collect();

        let mut square_table = vec![BitBoard::empty(); 4096];
        for i in 0usize..occupancies.len() {
            let key = magic_hash(magic, occupancies[i]);
            square_table[key] = attacks[i];
        }
        table.push(square_table);
    }

    table
}

#[cfg(test)]
mod tests {
    use super::AttackTable;
    use crate::core::bitboard::BitBoard;
    use crate::prelude::*;

    #[test]
    fn test_pawn_attack() {
        let table = AttackTable::build();

        let mut attacks_white = table.pawn_attacks(B2, Color::White);
        assert_eq!(attacks_white.pop_lowest_set(), Some(A3));
        assert_eq!(attacks_white.pop_lowest_set(), Some(C3));
        assert!(attacks_white.is_empty());

        let mut attacks_black = table.pawn_attacks(B2, Color::Black);
        assert_eq!(attacks_black.pop_lowest_set(), Some(A1));
        assert_eq!(attacks_black.pop_lowest_set(), Some(C1));
        assert!(attacks_black.is_empty());
    }

    #[test]
    fn test_knight_attack() {
        let table = AttackTable::build();

        let mut knight_attacks1 = table.knight_attacks(F7);
        assert_eq!(knight_attacks1.pop_lowest_set(), Some(E5));
        assert_eq!(knight_attacks1.pop_lowest_set(), Some(G5));
        assert_eq!(knight_attacks1.pop_lowest_set(), Some(D6));
        assert_eq!(knight_attacks1.pop_lowest_set(), Some(H6));
        assert_eq!(knight_attacks1.pop_lowest_set(), Some(D8));
        assert_eq!(knight_attacks1.pop_lowest_set(), Some(H8));
        assert!(knight_attacks1.is_empty());

        let mut knight_attacks2 = table.knight_attacks(A8);
        assert_eq!(knight_attacks2.pop_lowest_set(), Some(B6));
        assert_eq!(knight_attacks2.pop_lowest_set(), Some(C7));
        assert!(knight_attacks2.is_empty());

        let mut knight_attacks3 = table.knight_attacks(D5);
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(C3));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(E3));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(B4));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(F4));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(B6));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(F6));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(C7));
        assert_eq!(knight_attacks3.pop_lowest_set(), Some(E7));
        assert!(knight_attacks1.is_empty());
    }

    #[test]
    fn test_king_attack() {
        let table = AttackTable::build();

        let mut king_attacks1 = table.king_attacks(A8);
        assert_eq!(king_attacks1.pop_lowest_set(), Some(A7));
        assert_eq!(king_attacks1.pop_lowest_set(), Some(B7));
        assert_eq!(king_attacks1.pop_lowest_set(), Some(B8));
        assert!(king_attacks1.is_empty());

        let mut king_attacks2 = table.king_attacks(C3);
        assert_eq!(king_attacks2.pop_lowest_set(), Some(B2));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(C2));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(D2));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(B3));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(D3));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(B4));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(C4));
        assert_eq!(king_attacks2.pop_lowest_set(), Some(D4));
        assert!(king_attacks2.is_empty());
    }

    #[test]
    fn test_bishop_mask() {
        let table = AttackTable::build();

        let mut bishop_moves = table.bishop_mask(D4);
        assert_eq!(bishop_moves.pop_lowest_set(), Some(A1));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(G1));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(B2));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(F2));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(C3));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(E3));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(C5));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(E5));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(B6));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(F6));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(A7));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(G7));
        assert_eq!(bishop_moves.pop_lowest_set(), Some(H8));
        assert!(bishop_moves.is_empty());
    }

    #[test]
    fn test_rook_mask() {
        let table = AttackTable::build();

        let mut rook_moves = table.rook_mask(D4);
        assert_eq!(rook_moves.pop_lowest_set(), Some(D1));
        assert_eq!(rook_moves.pop_lowest_set(), Some(D2));
        assert_eq!(rook_moves.pop_lowest_set(), Some(D3));
        assert_eq!(rook_moves.pop_lowest_set(), Some(A4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(B4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(C4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(E4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(F4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(G4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(H4));
        assert_eq!(rook_moves.pop_lowest_set(), Some(D5));
        assert_eq!(rook_moves.pop_lowest_set(), Some(D6));
        assert_eq!(rook_moves.pop_lowest_set(), Some(D7));
        assert_eq!(rook_moves.pop_lowest_set(), Some(D8));
        assert!(rook_moves.is_empty());
    }

    #[test]
    fn test_queen_mask() {
        let table = AttackTable::build();

        let mut queen_moves = table.queen_mask(C6);
        assert_eq!(queen_moves.pop_lowest_set(), Some(C1));
        assert_eq!(queen_moves.pop_lowest_set(), Some(H1));
        assert_eq!(queen_moves.pop_lowest_set(), Some(C2));
        assert_eq!(queen_moves.pop_lowest_set(), Some(G2));
        assert_eq!(queen_moves.pop_lowest_set(), Some(C3));
        assert_eq!(queen_moves.pop_lowest_set(), Some(F3));
        assert_eq!(queen_moves.pop_lowest_set(), Some(A4));
        assert_eq!(queen_moves.pop_lowest_set(), Some(C4));
        assert_eq!(queen_moves.pop_lowest_set(), Some(E4));
        assert_eq!(queen_moves.pop_lowest_set(), Some(B5));
        assert_eq!(queen_moves.pop_lowest_set(), Some(C5));
        assert_eq!(queen_moves.pop_lowest_set(), Some(D5));
        assert_eq!(queen_moves.pop_lowest_set(), Some(A6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(B6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(D6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(E6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(F6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(G6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(H6));
        assert_eq!(queen_moves.pop_lowest_set(), Some(B7));
        assert_eq!(queen_moves.pop_lowest_set(), Some(C7));
        assert_eq!(queen_moves.pop_lowest_set(), Some(D7));
        assert_eq!(queen_moves.pop_lowest_set(), Some(A8));
        assert_eq!(queen_moves.pop_lowest_set(), Some(C8));
        assert_eq!(queen_moves.pop_lowest_set(), Some(E8));
        assert!(queen_moves.is_empty());
    }

    #[test]
    fn test_bishop_blocks() {
        let table = AttackTable::build();

        let mut bishop_blocks = table.bishop_blocks(D4);
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(B2));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(F2));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(C3));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(E3));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(C5));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(E5));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(B6));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(F6));
        assert_eq!(bishop_blocks.pop_lowest_set(), Some(G7));
        assert!(bishop_blocks.is_empty());
    }

    #[test]
    fn test_rook_blocks() {
        let table = AttackTable::build();

        let mut rook_blocks = table.rook_blocks(D4);
        assert_eq!(rook_blocks.pop_lowest_set(), Some(D2));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(D3));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(B4));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(C4));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(E4));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(F4));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(G4));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(D5));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(D6));
        assert_eq!(rook_blocks.pop_lowest_set(), Some(D7));
        assert!(rook_blocks.is_empty());
    }

    #[test]
    fn test_queen_blocks() {
        let table = AttackTable::build();

        let mut queen_blocks = table.queen_blocks(C6);
        assert_eq!(queen_blocks.pop_lowest_set(), Some(C2));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(G2));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(C3));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(F3));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(C4));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(E4));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(B5));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(C5));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(D5));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(B6));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(D6));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(E6));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(F6));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(G6));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(B7));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(C7));
        assert_eq!(queen_blocks.pop_lowest_set(), Some(D7));
        assert!(queen_blocks.is_empty());
    }

    #[test]
    fn test_bishop_attacks() {
        let table = AttackTable::build();

        let mut occupancy1 = BitBoard::empty();
        occupancy1.set(F1);
        occupancy1.set(B2);
        occupancy1.set(D2);
        occupancy1.set(C3);
        occupancy1.set(B4);
        occupancy1.set(C5);
        occupancy1.set(F6);

        let mut bishop_attacks1 = table.bishop_attacks(C3, occupancy1);
        assert_eq!(bishop_attacks1.pop_lowest_set(), Some(B2));
        assert_eq!(bishop_attacks1.pop_lowest_set(), Some(D2));
        assert_eq!(bishop_attacks1.pop_lowest_set(), Some(B4));
        assert_eq!(bishop_attacks1.pop_lowest_set(), Some(D4));
        assert_eq!(bishop_attacks1.pop_lowest_set(), Some(E5));
        assert_eq!(bishop_attacks1.pop_lowest_set(), Some(F6));
        assert!(bishop_attacks1.is_empty());

        let mut occupancy2 = BitBoard::empty();
        occupancy2.set(B7);
        occupancy2.set(A8);

        let mut bishop_attacks2 = table.bishop_attacks(A8, occupancy2);
        assert_eq!(bishop_attacks2.pop_lowest_set(), Some(B7));
        assert!(bishop_attacks2.is_empty());
    }

    #[test]
    fn test_rook_attacks() {
        let table = AttackTable::build();

        let mut occupancy1 = BitBoard::empty();
        occupancy1.set(C2);
        occupancy1.set(E3);
        occupancy1.set(A5);
        occupancy1.set(C5);
        occupancy1.set(G5);
        occupancy1.set(C6);
        occupancy1.set(F7);

        let mut rook_attacks1 = table.rook_attacks(C5, occupancy1);
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(C2));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(C3));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(C4));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(A5));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(B5));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(D5));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(E5));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(F5));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(G5));
        assert_eq!(rook_attacks1.pop_lowest_set(), Some(C6));
        assert!(rook_attacks1.is_empty());

        let mut occupancy2 = BitBoard::empty();
        occupancy2.set(A7);
        occupancy2.set(A8);
        occupancy2.set(B8);

        let mut rook_attacks2 = table.rook_attacks(A8, occupancy2);
        assert_eq!(rook_attacks2.pop_lowest_set(), Some(A7));
        assert_eq!(rook_attacks2.pop_lowest_set(), Some(B8));
        assert!(rook_attacks2.is_empty());
    }

    #[test]
    fn test_queen_attacks() {
        let table = AttackTable::build();

        let mut occupancy = BitBoard::empty();
        occupancy.set(G1);
        occupancy.set(B2);
        occupancy.set(E3);
        occupancy.set(G3);
        occupancy.set(C4);
        occupancy.set(D5);
        occupancy.set(E5);
        occupancy.set(H5);
        occupancy.set(B6);
        occupancy.set(F6);
        occupancy.set(C7);
        occupancy.set(E7);

        let mut queen_attacks = table.queen_attacks(E5, occupancy);
        assert_eq!(queen_attacks.pop_lowest_set(), Some(B2));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(C3));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(E3));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(G3));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(D4));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(E4));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(F4));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(D5));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(F5));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(G5));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(H5));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(D6));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(E6));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(F6));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(C7));
        assert_eq!(queen_attacks.pop_lowest_set(), Some(E7));
        assert!(queen_attacks.is_empty());
    }
}
