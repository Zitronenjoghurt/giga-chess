use crate::core::zobrist::ZobristKeys;
use crate::error::{FenError, FenResult};
use crate::prelude::*;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
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
    pub hash: u64,
}

impl Default for Position {
    fn default() -> Self {
        Self::from_board(ChessBoard::default())
    }
}

impl Position {
    pub fn from_board(board: ChessBoard) -> Self {
        let pos = Self {
            board,
            side_to_move: Color::White,
            castling_rights: CastlingRights::default(),
            en_passant_square: None,
            half_moves: 0,
            full_moves: 1,
            hash: 0,
        };
        Self {
            hash: ZobristKeys::full_hash(&pos),
            ..pos
        }
    }

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

        let mut hash = self.hash;

        match flags {
            MoveFlags::EnPassant => {
                let capture_square = to.pawn_push(opponent);
                self.board.clear(Piece::Pawn, opponent, capture_square);
                hash ^= ZobristKeys::piece_key(Piece::Pawn, opponent, capture_square);
            }
            _ if flags.is_capture() => {
                if let Some(captured) = self.board.piece_at_with_color(to, opponent) {
                    self.board.clear(captured, opponent, to);
                    hash ^= ZobristKeys::piece_key(captured, opponent, to);
                }
            }
            _ => {}
        }

        match flags {
            MoveFlags::KingCastle => {
                let (kf, kt, rf, rt) = color.kingside_castle_squares();
                self.board.move_piece(Piece::King, color, kf, kt);
                self.board.move_piece(Piece::Rook, color, rf, rt);
                hash ^= ZobristKeys::piece_key(Piece::King, color, kf);
                hash ^= ZobristKeys::piece_key(Piece::King, color, kt);
                hash ^= ZobristKeys::piece_key(Piece::Rook, color, rf);
                hash ^= ZobristKeys::piece_key(Piece::Rook, color, rt);
            }
            MoveFlags::QueenCastle => {
                let (kf, kt, rf, rt) = color.queenside_castle_squares();
                self.board.move_piece(Piece::King, color, kf, kt);
                self.board.move_piece(Piece::Rook, color, rf, rt);
                hash ^= ZobristKeys::piece_key(Piece::King, color, kf);
                hash ^= ZobristKeys::piece_key(Piece::King, color, kt);
                hash ^= ZobristKeys::piece_key(Piece::Rook, color, rf);
                hash ^= ZobristKeys::piece_key(Piece::Rook, color, rt);
            }
            _ if flags.is_promotion() => {
                let promo = flags.promotion_piece().unwrap();
                self.board.clear(piece, color, from);
                self.board.set(promo, color, to);
                hash ^= ZobristKeys::piece_key(piece, color, from);
                hash ^= ZobristKeys::piece_key(promo, color, to);
            }
            _ => {
                self.board.move_piece(piece, color, from, to);
                hash ^= ZobristKeys::piece_key(piece, color, from);
                hash ^= ZobristKeys::piece_key(piece, color, to);
            }
        }

        let old_ep = self.en_passant_square;
        let old_castling = self.castling_rights;

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

        hash ^= ZobristKeys::side_key();

        let new_castling = self.castling_rights;
        if old_castling != new_castling {
            hash ^= ZobristKeys::castling_key(&old_castling);
            hash ^= ZobristKeys::castling_key(&new_castling);
        }

        if let Some(sq) = old_ep {
            hash ^= ZobristKeys::ep_key(sq);
        }
        if let Some(sq) = self.en_passant_square {
            hash ^= ZobristKeys::ep_key(sq);
        }

        self.hash = hash;
        self
    }

    pub fn pretty_grid(&self) -> String {
        self.board.pretty_grid()
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
    type Err = FenError;

    fn from_str(s: &str) -> FenResult<Self> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 6 {
            return Err(FenError::InvalidPosition(
                "Must have 6 whitespace-separated parts".into(),
            ));
        }

        let board = ChessBoard::from_str(parts[0]).map_err(|err| {
            FenError::InvalidPosition(format!("Invalid board representation: {}", err))
        })?;

        let side_to_move = Color::from_str(parts[1])
            .map_err(|err| FenError::InvalidPosition(format!("Invalid side to move: {}", err)))?;

        let castling_rights = CastlingRights::from_str(parts[2]).map_err(|err| {
            FenError::InvalidPosition(format!("Invalid castling rights: {}", err))
        })?;

        let en_passant_square = Some(parts[3])
            .filter(|&square_str| square_str != "-")
            .map(Square::from_str)
            .transpose()
            .map_err(|err| {
                FenError::InvalidPosition(format!("Invalid en passant square: {}", err))
            })?;

        let half_moves = parts[4].parse::<u8>().map_err(|err| {
            FenError::InvalidPosition(format!("Invalid half-move count: {}", err))
        })?;

        let full_moves = parts[5].parse::<u16>().map_err(|err| {
            FenError::InvalidPosition(format!("Invalid full-move count: {}", err))
        })?;

        let pos = Self {
            board,
            side_to_move,
            castling_rights,
            en_passant_square,
            half_moves,
            full_moves,
            hash: 0,
        };
        Ok(Self {
            hash: ZobristKeys::full_hash(&pos),
            ..pos
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::core::position::Position;
    use crate::core::zobrist::ZobristKeys;
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

        let mut pos = Position::from_board(board);
        pos.half_moves = 1;
        pos.full_moves = 123;
        pos.castling_rights = CastlingRights::none();
        pos.hash = ZobristKeys::full_hash(&pos);

        let fen_string = pos.to_string();
        assert_eq!(fen_string, "4k1K1/7P/8/1p1q4/1p6/8/1b6/8 w - - 1 123");

        let loaded = Position::from_str(&fen_string).unwrap();
        assert_eq!(loaded, pos);
    }
}
