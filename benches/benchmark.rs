use criterion::{black_box, criterion_group, criterion_main, Criterion};
use giga_chess::chess_boards::{chess_board12::ChessBoard12, chess_board8::ChessBoard8};

fn chessboard_8() {
    let cb = ChessBoard8::default();
    black_box((
        cb.color_at_index(0),
        cb.color_at_index(63),
        cb.piece_at_index(0),
        cb.piece_at_index(63),
        cb.piece_and_color_at_index(0),
        cb.piece_and_color_at_index(63),
    ));
}

fn chessboard_12() {
    let cb = ChessBoard12::default();
    black_box((
        cb.color_at_index(0),
        cb.color_at_index(63),
        cb.piece_at_index(0),
        cb.piece_at_index(63),
        cb.piece_and_color_at_index(0),
        cb.piece_and_color_at_index(63),
    ));
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("chessboard8", |b| b.iter(chessboard_8));
    c.bench_function("chessboard12", |b| b.iter(chessboard_12));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
