use crate::engine::attack_table::{
    build_bishop_blocks, build_occupancy_variations, build_rook_blocks, calculate_bishop_attack,
    calculate_rook_attack,
};
use crate::game::bit_board::BitBoard;
use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const THREAD_COUNT: usize = 10;
const INDEX_SIZE_BITS: usize = 12;
const MAGIC_SHIFT: usize = 64 - INDEX_SIZE_BITS;
const TABLE_SIZE: usize = 1 << INDEX_SIZE_BITS;

/// Calculates the lookup table key for a given magic number and its blocking mask.
///
/// # Arguments
///
/// * `magic_number`: The precalculated magic number for the current square and sliding piece type.
/// * `occupied_blocking_mask`: The blocking mask containing all relevant occupied squares (squares that can actually block the piece from moving, so excluding the square itself and the ends of the movement lines).
///
/// returns: usize
pub fn magic_hash(magic_number: u64, occupied_blocking_mask: BitBoard) -> usize {
    (occupied_blocking_mask
        .get_value()
        .wrapping_mul(magic_number)
        >> MAGIC_SHIFT) as usize
}

pub fn find_bishop_magics() -> [u64; 64] {
    let bishop_blocks = build_bishop_blocks();
    let mut bishop_magics = [0u64; 64];
    for (square, mask) in bishop_blocks.iter().enumerate() {
        let magic = find_magic(square as u8, *mask, calculate_bishop_attack);
        bishop_magics[square] = magic;
    }
    bishop_magics
}

pub fn find_rook_magics() -> [u64; 64] {
    let rook_blocks = build_rook_blocks();
    let mut rook_magics = [0u64; 64];
    for (square, mask) in rook_blocks.iter().enumerate() {
        let magic = find_magic(square as u8, *mask, calculate_rook_attack);
        rook_magics[square] = magic;
    }
    rook_magics
}

fn precalculate_attacks(
    square: u8,
    occupancies: &Vec<BitBoard>,
    attack_fn: fn(u8, BitBoard) -> BitBoard,
) -> Vec<BitBoard> {
    occupancies
        .iter()
        .map(|occupancy| attack_fn(square, *occupancy))
        .collect()
}

fn find_magic(square: u8, block_mask: BitBoard, attack_fn: fn(u8, BitBoard) -> BitBoard) -> u64 {
    let occupancies = build_occupancy_variations(block_mask);
    let attacks = precalculate_attacks(square, &occupancies, attack_fn);
    let occupancy_count = occupancies.len();

    (0..THREAD_COUNT)
        .into_par_iter()
        .find_map_any(|_| {
            let mut rng = rand::rng();
            let mut table = [None; TABLE_SIZE];

            loop {
                let magic_number = rng.random::<u64>() & rng.random::<u64>() & rng.random::<u64>();
                table.fill(None);
                let mut valid = true;

                for i in 0..occupancy_count {
                    let occupancy = occupancies[i];
                    let attack = attacks[i];
                    let table_key = magic_hash(magic_number, occupancy);

                    match table[table_key] {
                        None => table[table_key] = Some(attack),
                        Some(existing) if existing != attack => {
                            valid = false;
                            break;
                        }
                        _ => {}
                    }
                }

                if valid {
                    return Some(magic_number);
                }
            }
        })
        .unwrap()
}
