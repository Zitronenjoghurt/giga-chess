use giga_chess::engine::Engine;
use giga_chess::game::bit_board::BitBoard;

fn main() {
    //let board = ChessBoard::default();
    //println!("{}", board);

    //let white_rooks = board.get_piece_bb(Piece::Rook, Color::White);
    //println!("{}", white_rooks);

    let engine = Engine::initialize();
    let mut occupancy = BitBoard::empty();
    occupancy.set_bit(27);
    occupancy.set_bit(45);
    let bishop_attack = engine.attack_table.get_bishop_attacks(27, occupancy);
    println!("{}", bishop_attack);
}
