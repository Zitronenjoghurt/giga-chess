use crate::game::castling_rights::CastlingRights;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::{ChessMove, ChessMoveType};
use crate::game::color::Color;
use crate::game::piece;
use crate::game::square::*;
use std::error::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GameState {
    pub board: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    /// En passant target square
    /// The square behind a pawn that just made a double move
    pub en_passant_square: Option<u8>,
    /// Half-move clock for 50-move rule
    /// Counts half-moves since last pawn move or capture
    pub half_moves: u8,
    /// Full-move counter, incremented after black's move
    pub full_moves: u16,
    // ToDo: Threefold repetition
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            board: ChessBoard::default(),
            side_to_move: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_moves: 0,
            full_moves: 1,
        }
    }
}

impl GameState {
    pub fn play_move(&mut self, chess_move: ChessMove) {
        let move_from = chess_move.get_from();
        let move_to = chess_move.get_to();
        let move_type = chess_move.get_type();
        let Some(moved_piece) = self
            .board
            .get_piece_at_with_color(move_from, self.side_to_move)
        else {
            return;
        };

        self.board = self.board.play_move(chess_move, self.side_to_move);

        if moved_piece == piece::Piece::Pawn || move_type.is_capture() {
            self.half_moves = 0;
        } else {
            self.half_moves = self.half_moves.wrapping_add(1);
        }

        if self.side_to_move == Color::Black {
            self.full_moves = self.full_moves.wrapping_add(1);
        }

        if move_type == ChessMoveType::DoublePawnPush {
            self.en_passant_square = match self.side_to_move {
                Color::White => Some(move_from + 8),
                Color::Black => Some(move_from - 8),
            }
        } else {
            self.en_passant_square = None;
        }

        if self.castling_rights.white_king_side
            && (move_from == E1 || move_from == H1 || move_to == H1)
        {
            self.castling_rights.white_king_side = false;
        }
        if self.castling_rights.white_queen_side
            && (move_from == E1 || move_from == A1 || move_to == A1)
        {
            self.castling_rights.white_queen_side = false;
        }
        if self.castling_rights.black_king_side
            && (move_from == E8 || move_from == H8 || move_to == H8)
        {
            self.castling_rights.black_king_side = false;
        }
        if self.castling_rights.black_queen_side
            && (move_from == E8 || move_from == A8 || move_to == A8)
        {
            self.castling_rights.black_queen_side = false;
        }

        self.side_to_move = self.side_to_move.opposite();
    }

    pub fn get_fen_string(&self) -> String {
        let mut fen_string = self.board.get_fen_string();
        fen_string.push(' ');
        fen_string.push(self.side_to_move.get_fen_char());

        fen_string.push(' ');
        fen_string.push_str(&self.castling_rights.get_fen_string());

        fen_string.push(' ');
        if let Some(en_passant_square) = self.en_passant_square {
            let square = Square::new(en_passant_square);
            fen_string.push_str(&square.to_string().to_lowercase());
        } else {
            fen_string.push('-');
        }

        fen_string.push(' ');
        fen_string.push_str(&self.half_moves.to_string());

        fen_string.push(' ');
        fen_string.push_str(&self.full_moves.to_string());

        fen_string
    }

    pub fn from_fen_string(fen_string: &str) -> Result<Self, Box<dyn Error>> {
        let parts: Vec<&str> = fen_string.split(' ').collect();
        if parts.len() != 6 {
            return Err("Must have 6 whitespace-separated parts".into());
        }

        let board = ChessBoard::from_fen_string(parts[0])
            .map_err(|err| format!("Invalid board representation: {}", err))?;

        let side_to_move =
            Color::try_from(parts[1]).map_err(|err| format!("Invalid side to move: {}", err))?;

        let castling_rights = CastlingRights::try_from(parts[2])
            .map_err(|err| format!("Invalid castling rights: {}", err))?;

        let en_passant_square = Some(parts[3])
            .filter(|&square_str| square_str != "-")
            .map(Square::try_from)
            .transpose()
            .map_err(|err| format!("Invalid en passant square: {}", err))?
            .map(|square| square.get_value());

        let half_moves = parts[4]
            .parse::<u8>()
            .map_err(|err| format!("Invalid half-move count: {}", err))?;

        let full_moves = parts[5]
            .parse::<u16>()
            .map_err(|err| format!("Invalid full-move count: {}", err))?;

        Ok(Self {
            board,
            side_to_move,
            castling_rights,
            en_passant_square,
            half_moves,
            full_moves,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::game::castling_rights::CastlingRights;
    use crate::game::chess_board::ChessBoard;
    use crate::game::color::Color;
    use crate::game::piece::Piece;
    use crate::game::square::*;
    use crate::game::state::GameState;

    #[test]
    fn test_fen() {
        let mut board = ChessBoard::empty();
        board.set_piece(Piece::King, Color::Black, E8);
        board.set_piece(Piece::King, Color::White, G8);
        board.set_piece(Piece::Pawn, Color::White, H7);
        board.set_piece(Piece::Pawn, Color::Black, B5);
        board.set_piece(Piece::Queen, Color::Black, D5);
        board.set_piece(Piece::Pawn, Color::Black, B4);
        board.set_piece(Piece::Bishop, Color::Black, B2);

        let game = GameState {
            board,
            side_to_move: Color::White,
            castling_rights: CastlingRights::none(),
            en_passant_square: None,
            half_moves: 1,
            full_moves: 123,
        };

        let fen_string = game.get_fen_string();
        assert_eq!(fen_string, "4k1K1/7P/8/1p1q4/1p6/8/1b6/8 w - - 1 123");

        let loaded_game = GameState::from_fen_string(&fen_string).unwrap();
        assert_eq!(loaded_game, game);
    }
}
