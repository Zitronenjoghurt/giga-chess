use criterion::{criterion_group, criterion_main, Criterion};
use giga_chess::bitboard::BitBoard;

// It makes no sense to benchmark this, it's just for testing criterion, lol
fn bitboard_get_bit() -> (bool, bool, bool, bool) {
    let b = BitBoard(0b0101);
    (b.get_bit(0), b.get_bit(1), b.get_bit(2), b.get_bit(3))
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("bb get_bit", |b| b.iter(bitboard_get_bit));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
