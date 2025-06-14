use crate::engine::Engine;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::{ChessMove, ChessMoveType};
use crate::game::color::Color;
use crate::game::piece::Piece;
use crate::game::square::Square;
use crate::game::state::GameState;
use crate::game::status::GameStatus;
use std::sync::Arc;

pub fn parse_move_to_algebraic_notation(
    engine: &Arc<Engine>,
    game_state: &GameState,
    chess_move: &ChessMove,
    color_to_move: Color,
) -> Option<String> {
    let move_from = Square::new(chess_move.get_from());
    let move_to = Square::new(chess_move.get_to());
    let move_type = chess_move.get_type();

    match move_type {
        ChessMoveType::KingCastle => return Some("O-O".to_string()),
        ChessMoveType::QueenCastle => return Some("O-O-O".to_string()),
        ChessMoveType::DoublePawnPush => return Some(move_to.to_string().to_lowercase()),
        _ => {}
    }

    let source_piece = game_state
        .board
        .get_piece_at_with_color(move_from.get_value(), color_to_move)?;

    let mut notation = String::new();
    match source_piece {
        Piece::Pawn => {
            if move_type.is_capture() || move_type == ChessMoveType::EnPassant {
                notation.push(move_from.get_file_char().to_ascii_lowercase());
                notation.push('x');
            }
            notation.push_str(&move_to.to_string().to_lowercase());
        }
        _ => {
            notation.push(source_piece.get_char());

            let (file, rank) = get_disambiguation(
                engine,
                &game_state.board,
                source_piece,
                color_to_move,
                move_from,
                move_to,
            );

            if let Some(file) = file {
                if let Some(push_char) = match file {
                    1 => Some('a'),
                    2 => Some('b'),
                    3 => Some('c'),
                    4 => Some('d'),
                    5 => Some('e'),
                    6 => Some('f'),
                    7 => Some('g'),
                    8 => Some('h'),
                    _ => None,
                } {
                    notation.push(push_char);
                }
            }

            if let Some(rank) = rank {
                notation.push_str(&rank.to_string())
            }

            if move_type.is_capture() {
                notation.push('x');
            }

            notation.push_str(&move_to.to_string().to_lowercase());
        }
    }

    if let Some(promotion_piece) = move_type.promotion_piece() {
        notation.push('=');
        notation.push(promotion_piece.get_char());
    }

    let mut new_game_state = game_state.clone();
    new_game_state.play_move(*chess_move);

    let (_, status) = engine.generate_moves(&new_game_state);
    if status == GameStatus::Checkmate {
        notation.push('#');
    } else if engine.is_in_check(new_game_state.board, color_to_move.opposite()) {
        notation.push('+')
    }

    Some(notation)
}

fn get_disambiguation(
    engine: &Arc<Engine>,
    board: &ChessBoard,
    piece: Piece,
    color: Color,
    move_from: Square,
    move_to: Square,
) -> (Option<u8>, Option<u8>) {
    if piece == Piece::King || piece == Piece::Pawn {
        return (None, None);
    }

    let piece_bb = board.get_piece_bb(piece, color);
    let occupied = board.get_occupied_bb();

    // Contains all squares of the same pieces that can also move to the square
    let relevant_mask = if piece == Piece::Knight {
        piece_bb & engine.attack_table.get_knight_attacks(move_to.get_value())
    } else if piece == Piece::Bishop {
        piece_bb
            & engine
                .attack_table
                .get_bishop_attacks(move_to.get_value(), occupied)
    } else if piece == Piece::Rook {
        piece_bb
            & engine
                .attack_table
                .get_rook_attacks(move_to.get_value(), occupied)
    } else {
        piece_bb
            & engine
                .attack_table
                .get_queen_attacks(move_to.get_value(), occupied)
    };

    let mut conflicting_squares = Vec::new();
    for index in relevant_mask.iter_set_bits() {
        if index != move_from.get_value() {
            conflicting_squares.push(Square::new(index));
        }
    }

    if conflicting_squares.is_empty() {
        return (None, None);
    }

    let from_file = move_from.get_file();
    let from_rank = move_from.get_rank();

    let file_conflicts = conflicting_squares
        .iter()
        .any(|sq| sq.get_file() == from_file);
    if !file_conflicts {
        return (Some(from_file), None);
    }

    let rank_conflicts = conflicting_squares
        .iter()
        .any(|sq| sq.get_rank() == from_rank);
    if !rank_conflicts {
        return (None, Some(from_rank));
    }

    (Some(from_file), Some(from_rank))
}
