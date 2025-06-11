use criterion::{criterion_group, criterion_main, Criterion};
use giga_chess::types::bitboard::BitBoard;
use std::hint::black_box;

// This benchmark does not provide any meaningful results (performance on bit-level fluctuates a lot).
// It is just used as a reference for future benchmarks.
fn benchmark_bitboard_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("bitboard_operations");

    group.bench_function("get_bit_first", |b| {
        let bitboard = BitBoard::new(0xFFFFFFFFFFFFFFFF);
        b.iter(|| black_box(bitboard.get_bit(black_box(0))))
    });

    group.bench_function("get_bit_middle", |b| {
        let bitboard = BitBoard::new(0xFFFFFFFFFFFFFFFF);
        b.iter(|| black_box(bitboard.get_bit(black_box(32))))
    });

    group.bench_function("get_bit_last", |b| {
        let bitboard = BitBoard::new(0xFFFFFFFFFFFFFFFF);
        b.iter(|| black_box(bitboard.get_bit(black_box(63))))
    });

    group.bench_function("set_all_bits", |b| {
        b.iter(|| {
            let mut bitboard = BitBoard::empty();
            for i in 0..64u8 {
                bitboard.set_bit(black_box(i));
            }
            black_box(bitboard)
        })
    });

    group.bench_function("clear_all_bits", |b| {
        b.iter(|| {
            let mut bitboard = BitBoard::new(0xFFFFFFFFFFFFFFFF);
            for i in 0..64u8 {
                bitboard.clear_bit(black_box(i));
            }
            black_box(bitboard)
        })
    });

    group.bench_function("mixed_operations", |b| {
        b.iter(|| {
            let mut bitboard = BitBoard::empty();
            for i in 0..32u8 {
                bitboard.set_bit(black_box(i * 2));
            }
            for i in 0..32u8 {
                bitboard.clear_bit(black_box(i * 2 + 1));
            }
            let mut count = 0;
            for i in 0..64u8 {
                if bitboard.get_bit(i) {
                    count += 1;
                }
            }
            black_box((bitboard, count))
        })
    });

    group.finish();
}

criterion_group!(benches, benchmark_bitboard_operations,);
criterion_main!(benches);
