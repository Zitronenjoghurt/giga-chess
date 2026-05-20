use crate::core::position::Position;
use crate::error::{ChessError, ChessResult};
use crate::moves::generator::MoveGenerator;
use crate::moves::list::MoveList;
use crate::prelude::{ChessMove, Piece};

pub fn move_to_san(pos: &Position, mv: ChessMove, legal_moves: &MoveList) -> ChessResult<String> {
    let flags = mv.flags();
    if flags.is_kingside_castle() {
        return Ok("O-O".to_string());
    }
    if flags.is_queenside_castle() {
        return Ok("O-O-O".to_string());
    }

    let (piece, color) = pos
        .board
        .piece_at(mv.from())
        .ok_or(ChessError::IllegalMove)?;
    let is_capture = pos.board.piece_at(mv.to()).is_some() || flags.is_en_passant();
    let mut san = String::new();

    if piece == Piece::Pawn {
        if is_capture {
            san.push(mv.from().file_char());
        }
    } else {
        san.push(piece.char());

        let ambiguous: Vec<_> = legal_moves
            .iter()
            .filter(|other| {
                other.to() == mv.to()
                    && other.from() != mv.from()
                    && pos.board.piece_at(other.from()) == Some((piece, color))
            })
            .collect();

        if !ambiguous.is_empty() {
            if ambiguous
                .iter()
                .all(|m| m.from().file() != mv.from().file())
            {
                san.push(mv.from().file_char());
            } else if ambiguous
                .iter()
                .all(|m| m.from().rank() != mv.from().rank())
            {
                san.push(mv.from().rank_char());
            } else {
                san.push(mv.from().rank_char());
                san.push(mv.from().rank_char());
            }
        }
    }

    if is_capture {
        san.push('x');
    }

    san.push(mv.to().file_char());
    san.push(mv.to().rank_char());

    if let Some(promo) = flags.promotion_piece() {
        san.push('=');
        san.push(promo.char());
    }

    let new_pos = pos.make_move(mv);
    if MoveGenerator::get().is_in_check(&new_pos, new_pos.side_to_move) {
        let new_moves = MoveGenerator::get().generate(&new_pos);
        if new_moves.is_empty() {
            san.push('#');
        } else {
            san.push('+');
        }
    }

    Ok(san)
}

// ToDo: Parse from SAN
