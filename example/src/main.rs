use giga_chess::game::chess_board::ChessBoard;
use giga_chess::game::color::Color;
use giga_chess::game::piece::Piece;

fn main() {
    let board = ChessBoard::default();
    println!("{}", board);

    let white_rooks = board.get_piece_bb(Piece::Rook, Color::White);
    println!("{}", white_rooks);
}
