use crate::error::{ChessError, ChessResult};
use crate::prelude::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Position {
    pub board: ChessBoard,
    pub side_to_move: Color,
    pub castling_rights: CastlingRights,
    /// En passant target square
    /// The square behind a pawn that just made a double move
    pub en_passant_square: Option<Square>,
    /// Half-move clock for 50-move rule
    /// Counts half-moves since last pawn move or capture
    pub half_moves: u8,
    /// Full-move counter, incremented after black's move
    pub full_moves: u16,
    // ToDo: Threefold repetition
}

impl Default for Position {
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

impl Position {
    /// Applying a move assuming it's legal.
    pub fn make_move(mut self, mv: ChessMove) -> Self {
        let from = mv.from();
        let to = mv.to();
        let flags = mv.flags();
        let color = self.side_to_move;
        let opponent = color.opposite();

        let Some(piece) = self.board.piece_at_with_color(from, color) else {
            return self;
        };

        match flags {
            MoveFlags::EnPassant => {
                self.board
                    .clear(Piece::Pawn, opponent, to.pawn_push(opponent));
            }
            _ if flags.is_capture() => {
                if let Some(captured) = self.board.piece_at_with_color(to, opponent) {
                    self.board.clear(captured, opponent, to);
                }
            }
            _ => {}
        }

        match flags {
            MoveFlags::KingCastle => {
                let (king_from, king_to, rook_from, rook_to) = color.kingside_castle_squares();
                self.board
                    .move_piece(Piece::King, color, king_from, king_to);
                self.board
                    .move_piece(Piece::Rook, color, rook_from, rook_to);
            }
            MoveFlags::QueenCastle => {
                let (king_from, king_to, rook_from, rook_to) = color.queenside_castle_squares();
                self.board
                    .move_piece(Piece::King, color, king_from, king_to);
                self.board
                    .move_piece(Piece::Rook, color, rook_from, rook_to);
            }
            _ if flags.is_promotion() => {
                self.board.clear(piece, color, from);
                self.board.set(flags.promotion_piece().unwrap(), color, to);
            }
            _ => {
                self.board.move_piece(piece, color, from, to);
            }
        }

        self.en_passant_square = if flags == MoveFlags::DoublePawnPush {
            Some(from.pawn_push(color))
        } else {
            None
        };

        self.half_moves = if piece == Piece::Pawn || flags.is_capture() {
            0
        } else {
            self.half_moves.saturating_add(1)
        };

        if color == Color::Black {
            self.full_moves = self.full_moves.saturating_add(1);
        }

        self.castling_rights.update(from, to);
        self.side_to_move = opponent;

        self
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fen_string = self.board.to_string();
        fen_string.push(' ');
        fen_string.push_str(&self.side_to_move.to_string());

        fen_string.push(' ');
        fen_string.push_str(&self.castling_rights.to_string());

        fen_string.push(' ');
        if let Some(en_passant_square) = self.en_passant_square {
            fen_string.push_str(&en_passant_square.to_string().to_lowercase());
        } else {
            fen_string.push('-');
        }

        fen_string.push(' ');
        fen_string.push_str(&self.half_moves.to_string());

        fen_string.push(' ');
        fen_string.push_str(&self.full_moves.to_string());

        write!(f, "{fen_string}")
    }
}

impl FromStr for Position {
    type Err = ChessError;

    fn from_str(s: &str) -> ChessResult<Self> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 6 {
            return Err(ChessError::InvalidPosition(
                "Must have 6 whitespace-separated parts".into(),
            ));
        }

        let board = ChessBoard::from_str(parts[0]).map_err(|err| {
            ChessError::InvalidPosition(format!("Invalid board representation: {}", err))
        })?;

        let side_to_move = Color::from_str(parts[1])
            .map_err(|err| ChessError::InvalidPosition(format!("Invalid side to move: {}", err)))?;

        let castling_rights = CastlingRights::from_str(parts[2]).map_err(|err| {
            ChessError::InvalidPosition(format!("Invalid castling rights: {}", err))
        })?;

        let en_passant_square = Some(parts[3])
            .filter(|&square_str| square_str != "-")
            .map(Square::from_str)
            .transpose()
            .map_err(|err| {
                ChessError::InvalidPosition(format!("Invalid en passant square: {}", err))
            })?;

        let half_moves = parts[4].parse::<u8>().map_err(|err| {
            ChessError::InvalidPosition(format!("Invalid half-move count: {}", err))
        })?;

        let full_moves = parts[5].parse::<u16>().map_err(|err| {
            ChessError::InvalidPosition(format!("Invalid full-move count: {}", err))
        })?;

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
    use crate::core::position::Position;
    use crate::prelude::*;
    use std::str::FromStr;

    #[test]
    fn test_fen() {
        let mut board = ChessBoard::empty();
        board.set(Piece::King, Color::Black, E8);
        board.set(Piece::King, Color::White, G8);
        board.set(Piece::Pawn, Color::White, H7);
        board.set(Piece::Pawn, Color::Black, B5);
        board.set(Piece::Queen, Color::Black, D5);
        board.set(Piece::Pawn, Color::Black, B4);
        board.set(Piece::Bishop, Color::Black, B2);

        let pos = Position {
            board,
            side_to_move: Color::White,
            castling_rights: CastlingRights::none(),
            en_passant_square: None,
            half_moves: 1,
            full_moves: 123,
        };

        let fen_string = pos.to_string();
        assert_eq!(fen_string, "4k1K1/7P/8/1p1q4/1p6/8/1b6/8 w - - 1 123");

        let loaded = Position::from_str(&fen_string).unwrap();
        assert_eq!(loaded, pos);
    }
}
