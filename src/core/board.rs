use crate::core::bitboard::BitBoard;
use crate::core::piece::{Color, Piece};
use crate::core::square::Square;
use crate::error::{FenError, FenResult};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const DEFAULT_BOARD: ChessBoard = ChessBoard([
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000),
    BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000),
    BitBoard::new(0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    BitBoard::new(0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
]);

type ChessBoardIterator<'a> = Box<dyn Iterator<Item = (Square, Option<(Piece, Color)>)> + 'a>;

/// A chess board containing 12 bitboards for each piece and color.
///
/// Square indexing starts with 0 at A1, 1 at B1, ... and ends with 63 at H8.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ChessBoard([BitBoard; 12]);

impl ChessBoard {
    /// Creates a new chess board from the given bitboards.
    ///
    /// The bit boards are indexed as follows:\
    /// 0: White Pawns\
    /// 1: White Knights\
    /// 2: White Bishops\
    /// 3: White Rooks\
    /// 4: White Queens\
    /// 5: White Kings\
    /// 6: Black Pawns\
    /// 7: Black Knights\
    /// 8: Black Bishops\
    /// 9: Black Rooks\
    /// 10: Black Queens\
    /// 11: Black Kings
    ///
    /// # Arguments
    ///
    /// * `bitboards`: 12 bitboards for each piece and color.
    ///
    /// Returns: [`ChessBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::core::bitboard::BitBoard;
    /// use giga_chess::prelude::*;
    ///
    /// let boards = [
    ///     BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_11111111_00000000),
    ///     BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_01000010),
    ///     BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00100100),
    ///     BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_10000001),
    ///     BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00001000),
    ///     BitBoard::new(0b00000000_00000000_00000000_00000000_00000000_00000000_00000000_00010000),
    ///     BitBoard::new(0b00000000_11111111_00000000_00000000_00000000_00000000_00000000_00000000),
    ///     BitBoard::new(0b01000010_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    ///     BitBoard::new(0b00100100_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    ///     BitBoard::new(0b10000001_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    ///     BitBoard::new(0b00001000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    ///     BitBoard::new(0b00010000_00000000_00000000_00000000_00000000_00000000_00000000_00000000),
    /// ];
    ///
    /// let chess_board = ChessBoard::new(boards);
    /// assert_eq!(chess_board.piece_at(A1), Some((Piece::Rook, Color::White)));
    /// assert_eq!(chess_board.piece_at(H8), Some((Piece::Rook, Color::Black)));
    /// assert_eq!(chess_board.piece_at(E2), Some((Piece::Pawn, Color::White)));
    /// assert_eq!(chess_board.piece_at(E3), None);
    /// ```
    pub const fn new(bitboards: [BitBoard; 12]) -> Self {
        Self(bitboards)
    }

    /// Creates a new [`ChessBoard`] without any pieces.
    ///
    /// Returns: [`ChessBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let chess_board = ChessBoard::empty();
    /// assert_eq!(chess_board.piece_at(A1), None);
    /// assert_eq!(chess_board.piece_at(H8), None);
    /// assert_eq!(chess_board.piece_at(E2), None);
    /// assert_eq!(chess_board.piece_at(E3), None);
    /// ```
    pub const fn empty() -> Self {
        Self([BitBoard::empty(); 12])
    }

    /// Creates a new [`ChessBoard`] with the standard placement of pieces.
    ///
    /// Returns: [`ChessBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let chess_board = ChessBoard::default();
    /// assert_eq!(chess_board.piece_at(A1), Some((Piece::Rook, Color::White)));
    /// assert_eq!(chess_board.piece_at(H8), Some((Piece::Rook, Color::Black)));
    /// assert_eq!(chess_board.piece_at(E2), Some((Piece::Pawn, Color::White)));
    /// assert_eq!(chess_board.piece_at(E3), None);
    /// ```
    pub const fn default() -> Self {
        DEFAULT_BOARD
    }

    /// Retrieve the [`BitBoard`] of the respective piece and color.
    ///
    /// # Arguments
    /// * `piece`: The piece to retrieve the bit board for
    /// * `color`: The color to retrieve the bit board for
    ///
    /// Returns: [`BitBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let chess_board = ChessBoard::default();
    /// let white_rooks = chess_board.piece_bb(Piece::Rook, Color::White);
    ///
    /// assert!(white_rooks.is_set(H1));
    /// ```
    pub fn piece_bb(&self, piece: Piece, color: Color) -> BitBoard {
        self.0[piece as usize + (color as usize * 6)]
    }

    /// Retrieve a mutable reference to the [`BitBoard`] of the respective piece and color.
    ///
    /// # Arguments
    /// * `piece`: The piece to retrieve the bit board for
    /// * `color`: The color to retrieve the bit board for
    ///
    /// Returns: &mut [`BitBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let mut chess_board = ChessBoard::empty();
    /// let white_rooks = chess_board.piece_bb_mut(Piece::Rook, Color::White);
    /// white_rooks.set(B5);
    ///
    /// assert_eq!(chess_board.piece_at(B5), Some((Piece::Rook, Color::White)));
    /// ```
    pub fn piece_bb_mut(&mut self, piece: Piece, color: Color) -> &mut BitBoard {
        &mut self.0[piece as usize + (color as usize * 6)]
    }

    /// Retrieve a bitboard which contains all piece positions of the respective color (occupation mask).
    ///
    /// # Arguments
    /// * `color`: The color to retrieve the occupation mask for
    ///
    /// Returns: [`BitBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let chess_board = ChessBoard::default();
    /// let white_occupation = chess_board.color_bb(Color::White);
    /// let black_occupation = chess_board.color_bb(Color::Black);
    ///
    /// assert_eq!(white_occupation.value(), 0x000000000000FFFF);
    /// assert_eq!(black_occupation.value(), 0xFFFF000000000000);
    /// ```
    pub fn color_bb(&self, color: Color) -> BitBoard {
        let base = color as usize * 6;
        self.0[base]
            | self.0[base + 1]
            | self.0[base + 2]
            | self.0[base + 3]
            | self.0[base + 4]
            | self.0[base + 5]
    }

    /// Retrieve a bitboard which contains all piece positions (total occupation mask).
    ///
    /// Returns: [`BitBoard`]
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let chess_board = ChessBoard::default();
    /// let occupation = chess_board.occupied_bb();
    ///
    /// assert_eq!(occupation.value(), 0xFFFF00000000FFFF);
    /// ```
    pub fn occupied_bb(&self) -> BitBoard {
        self.color_bb(Color::White) | self.color_bb(Color::Black)
    }

    /// Place a piece of a certain color on the specified square.
    ///
    /// # Arguments
    /// * `piece`: The piece to place
    /// * `color`: The color of the piece to place
    /// * `square`: Where to place the piece
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let mut chess_board = ChessBoard::empty();
    /// chess_board.set(Piece::Queen, Color::White, D5);
    ///
    /// assert_eq!(chess_board.piece_at(D5), Some((Piece::Queen, Color::White)));
    /// ```
    pub fn set(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bb_mut(piece, color).set(square);
    }

    /// Clear a piece of a certain color from the specified square.
    ///
    /// # Arguments
    /// * `piece`: The piece to clear
    /// * `color`: The color of the piece to clear
    /// * `square`: Where to clear the piece
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let mut chess_board = ChessBoard::default();
    /// chess_board.clear(Piece::King, Color::White, E1);
    ///
    /// assert_eq!(chess_board.piece_at(E1), None);
    /// ```
    pub fn clear(&mut self, piece: Piece, color: Color, square: Square) {
        self.piece_bb_mut(piece, color).clear(square);
    }

    /// Move a piece of a certain color from a specified square to another square.
    ///
    /// # Arguments
    /// * `piece`: The piece to move
    /// * `color`: The color of the piece to move
    /// * `from`: Where the piece is currently at
    /// * `to`: Where to move the piece to
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let mut chess_board = ChessBoard::default();
    /// chess_board.move_piece(Piece::Pawn, Color::White, E2, E4);
    ///
    /// assert_eq!(chess_board.piece_at(E2), None);
    /// assert_eq!(chess_board.piece_at(E4), Some((Piece::Pawn, Color::White)));
    /// ```
    pub fn move_piece(&mut self, piece: Piece, color: Color, from: Square, to: Square) {
        let bb = self.piece_bb_mut(piece, color);
        bb.clear(from);
        bb.set(to);
    }

    /// Retrieves the piece and its color at a specified square.
    ///
    /// # Arguments
    /// * `square`: The square to search for a piece at
    ///
    /// Returns: Option<([`Piece`], [`Color`])>
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let mut chess_board = ChessBoard::default();
    /// assert_eq!(chess_board.piece_at(B1), Some((Piece::Knight, Color::White)));
    /// assert_eq!(chess_board.piece_at(B3), None);
    /// ```
    pub fn piece_at(&self, square: Square) -> Option<(Piece, Color)> {
        for color in Color::ALL {
            if let Some(piece) = self.piece_at_with_color(square, color) {
                return Some((piece, color));
            }
        }
        None
    }

    /// Retrieves the piece with the specified color at the specified square.\
    /// This is faster than searching for the piece without knowing its color.
    ///
    /// # Arguments
    /// * `square`: The square to search for a piece at
    /// * `color`: The color of the piece to search for
    ///
    /// Returns: Option<[`Piece`]>
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let mut chess_board = ChessBoard::default();
    /// assert_eq!(chess_board.piece_at_with_color(B1, Color::White), Some(Piece::Knight));
    /// assert_eq!(chess_board.piece_at_with_color(B1, Color::Black), None);
    /// ```
    pub fn piece_at_with_color(&self, square: Square, color: Color) -> Option<Piece> {
        Piece::ALL
            .into_iter()
            .find(|&piece| self.piece_bb(piece, color).is_set(square))
    }

    /// Iterate over all squares of the chess board top to bottom, left to right.
    ///
    /// # Arguments
    /// * `perspective`: The perspective of the player from which to iterate over the squares
    ///
    /// Returns: Iterator<Item = (Square, Option<(Piece, Color)>)
    ///
    /// # Examples
    ///
    /// ```
    /// use giga_chess::prelude::*;
    ///
    /// let chess_board = ChessBoard::default();
    ///
    /// let mut result = String::new();
    /// for (square, piece_color) in chess_board.iter_top_bottom(Color::White) {
    ///     if let Some((piece, color)) = piece_color {
    ///         result.push_str(piece.icon(color));
    ///     } else if square.is_white() {
    ///         result.push_str("□ ");
    ///     } else {
    ///         result.push_str("■ ");
    ///     }
    ///     if square.is_right_edge() && !square.is_lower_edge() {
    ///         result.push_str("\n");
    ///     }
    /// }
    ///
    /// assert_eq!(result, "♜♞♝♛♚♝♞♜\n♟♟♟♟♟♟♟♟\n□ ■ □ ■ □ ■ □ ■ \n■ □ ■ □ ■ □ ■ □ \n□ ■ □ ■ □ ■ □ ■ \n■ □ ■ □ ■ □ ■ □ \n♙♙♙♙♙♙♙♙\n♖♘♗♕♔♗♘♖");
    /// ```
    pub fn iter_top_bottom(&'_ self, perspective: Color) -> ChessBoardIterator<'_> {
        match perspective {
            Color::White => Box::new(
                Square::iter_top_bottom().map(move |square| (square, self.piece_at(square))),
            ),
            Color::Black => Box::new(
                Square::iter_bottom_top().map(move |square| (square, self.piece_at(square))),
            ),
        }
    }

    /// Iterate over all squares of the chess board bottom to top, left to right.
    ///
    /// # Arguments
    /// * `perspective`: The perspective of the player from which to iterate over the squares
    ///
    /// Returns: Iterator<Item = (Square, Option<(Piece, Color)>)
    pub fn iter_bottom_top(&'_ self, perspective: Color) -> ChessBoardIterator<'_> {
        match perspective {
            Color::White => Box::new(
                Square::iter_bottom_top().map(move |square| (square, self.piece_at(square))),
            ),
            Color::Black => Box::new(
                Square::iter_top_bottom().map(move |square| (square, self.piece_at(square))),
            ),
        }
    }

    pub fn as_grid(&self) -> String {
        let mut grid = String::new();

        for (square, piece_color) in self.iter_top_bottom(Color::White) {
            if square.is_left_edge() {
                grid.push_str(format!("{} ", square.get_rank()).as_str());
            }

            if let Some((piece, color)) = piece_color {
                grid.push_str(format!("{} ", piece.icon(color)).as_str());
            } else if square.is_white() {
                grid.push_str("□ ");
            } else {
                grid.push_str("■ ");
            }

            if square.is_right_edge() {
                grid.push('\n');
            }
        }

        grid.push_str("  A B C D E F G H");

        grid
    }
}

impl FromStr for ChessBoard {
    type Err = FenError;

    fn from_str(s: &str) -> FenResult<Self> {
        let mut board = Self::empty();

        let mut rank = 8;
        let mut file = 1;
        for current_char in s.chars() {
            if current_char == '/' {
                rank -= 1;
                file = 1;
                continue;
            }

            if file > 8 {
                return Err(FenError::InvalidChessBoard(format!(
                    "File exceeds H in rank {rank}"
                )));
            }

            if let Ok(piece) = Piece::from_str(&current_char.to_string()) {
                let color = if current_char.is_ascii_uppercase() {
                    Color::White
                } else {
                    Color::Black
                };
                let square = Square::from_file_rank(file, rank);
                board.set(piece, color, square);
                file += 1;
            } else if let Some(count) = current_char.to_digit(10) {
                if count > 8 || file + count as u8 > 9 {
                    return Err(FenError::InvalidChessBoard(format!(
                        "Count exceeds 8 in rank {rank}"
                    )));
                }
                file += count as u8;
            } else {
                return Err(FenError::InvalidChessBoard(format!(
                    "Invalid character '{current_char}' in rank {rank}"
                )));
            }
        }

        Ok(board)
    }
}

impl Display for ChessBoard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut fen = String::new();

        for rank in (1..=8).rev() {
            let mut empty_count = 0;
            let mut rank_string = String::new();

            for file in 1..=8 {
                let square = Square::from_file_rank(file, rank);
                if let Some((piece, color)) = self.piece_at(square) {
                    if empty_count > 0 {
                        rank_string.push_str(&empty_count.to_string());
                        empty_count = 0;
                    }
                    rank_string.push(piece.fen_char(color));
                } else {
                    empty_count += 1;
                }
            }

            if empty_count > 0 {
                rank_string.push_str(&empty_count.to_string());
            }

            if rank > 1 {
                rank_string.push('/');
            }

            fen.push_str(&rank_string);
        }

        write!(f, "{fen}")
    }
}

#[cfg(test)]
mod tests {
    use crate::core::bitboard::BitBoard;
    use crate::prelude::*;

    const BOARDS: [BitBoard; 12] = [
        BitBoard::new(0b000000_000001),
        BitBoard::new(0b000000_000010),
        BitBoard::new(0b000000_000100),
        BitBoard::new(0b000000_001000),
        BitBoard::new(0b000000_010000),
        BitBoard::new(0b000000_100000),
        BitBoard::new(0b000001_000000),
        BitBoard::new(0b000010_000000),
        BitBoard::new(0b000100_000000),
        BitBoard::new(0b001000_000000),
        BitBoard::new(0b010000_000000),
        BitBoard::new(0b100000_000000),
    ];

    #[test]
    fn test_get_piece_bb() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(
            board.piece_bb(Piece::Pawn, Color::White).value(),
            0b000000_000001
        );
        assert_eq!(
            board.piece_bb(Piece::Pawn, Color::Black).value(),
            0b000001_000000
        );
        assert_eq!(
            board.piece_bb(Piece::Knight, Color::White).value(),
            0b000000_000010
        );
        assert_eq!(
            board.piece_bb(Piece::Knight, Color::Black).value(),
            0b000010_000000
        );
        assert_eq!(
            board.piece_bb(Piece::Bishop, Color::White).value(),
            0b000000_000100
        );
        assert_eq!(
            board.piece_bb(Piece::Bishop, Color::Black).value(),
            0b000100_000000
        );
        assert_eq!(
            board.piece_bb(Piece::Rook, Color::White).value(),
            0b000000_001000
        );
        assert_eq!(
            board.piece_bb(Piece::Rook, Color::Black).value(),
            0b001000_000000
        );
        assert_eq!(
            board.piece_bb(Piece::Queen, Color::White).value(),
            0b000000_010000
        );
        assert_eq!(
            board.piece_bb(Piece::Queen, Color::Black).value(),
            0b010000_000000
        );
        assert_eq!(
            board.piece_bb(Piece::King, Color::White).value(),
            0b000000_100000
        );
        assert_eq!(
            board.piece_bb(Piece::King, Color::Black).value(),
            0b100000_000000
        );
    }

    #[test]
    fn test_get_color_bb() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(board.color_bb(Color::White).value(), 0b000000_111111);
        assert_eq!(board.color_bb(Color::Black).value(), 0b111111_000000);
    }

    #[test]
    fn test_get_occupied_bb() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(board.occupied_bb().value(), 0b111111_111111);
    }

    #[test]
    fn test_set_piece() {
        let mut board = ChessBoard::empty();
        board.set(Piece::Pawn, Color::White, A1);
        assert_eq!(
            board.piece_bb(Piece::Pawn, Color::White).value(),
            0b000000_000001
        );
        board.set(Piece::Pawn, Color::Black, B1);
        assert_eq!(
            board.piece_bb(Piece::Pawn, Color::Black).value(),
            0b000000_000010
        );
    }

    #[test]
    fn test_clear_piece() {
        let mut board = ChessBoard::new(BOARDS);
        board.clear(Piece::Pawn, Color::White, A1);
        board.clear(Piece::Knight, Color::White, B1);
        board.clear(Piece::Bishop, Color::White, C1);
        board.clear(Piece::Rook, Color::White, D1);
        board.clear(Piece::Queen, Color::White, E1);
        board.clear(Piece::King, Color::White, F1);
        board.clear(Piece::Pawn, Color::Black, G1);
        board.clear(Piece::Knight, Color::Black, H1);
        board.clear(Piece::Bishop, Color::Black, A2);
        board.clear(Piece::Rook, Color::Black, B2);
        board.clear(Piece::Queen, Color::Black, C2);
        board.clear(Piece::King, Color::Black, D2);
        assert_eq!(board.occupied_bb().value(), 0);
    }

    #[test]
    fn test_get_piece_at() {
        let board = ChessBoard::new(BOARDS);
        assert_eq!(board.piece_at(A1).unwrap(), (Piece::Pawn, Color::White));
        assert_eq!(board.piece_at(B1).unwrap(), (Piece::Knight, Color::White));
        assert_eq!(board.piece_at(C1).unwrap(), (Piece::Bishop, Color::White));
        assert_eq!(board.piece_at(D1).unwrap(), (Piece::Rook, Color::White));
        assert_eq!(board.piece_at(E1).unwrap(), (Piece::Queen, Color::White));
        assert_eq!(board.piece_at(F1).unwrap(), (Piece::King, Color::White));
        assert_eq!(board.piece_at(G1).unwrap(), (Piece::Pawn, Color::Black));
        assert_eq!(board.piece_at(H1).unwrap(), (Piece::Knight, Color::Black));
        assert_eq!(board.piece_at(A2).unwrap(), (Piece::Bishop, Color::Black));
        assert_eq!(board.piece_at(B2).unwrap(), (Piece::Rook, Color::Black));
        assert_eq!(board.piece_at(C2).unwrap(), (Piece::Queen, Color::Black));
        assert_eq!(board.piece_at(D2).unwrap(), (Piece::King, Color::Black));
        assert_eq!(board.piece_at(E2), None);
    }

    // ToDo: Move this somewhere else
    //fn test_play_move() {
    //    let board = ChessBoard::default();
    //
    //    let m = ChessMove::new(C2, 0, ChessMoveType::DoublePawnPush);
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(C2), None);
    //    assert_eq!(board.piece_at(C4).unwrap(), (Piece::Pawn, Color::White));
    //
    //    let m = ChessMove::new(C4, C5, ChessMoveType::Quiet);
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(C4), None);
    //    assert_eq!(board.piece_at(C5).unwrap(), (Piece::Pawn, Color::White));
    //
    //    let m = ChessMove::new(D7, 0, ChessMoveType::DoublePawnPush);
    //    let board = board.play_move(m, Color::Black);
    //    assert_eq!(board.piece_at(D7), None);
    //    assert_eq!(board.piece_at(D5).unwrap(), (Piece::Pawn, Color::Black));
    //
    //    let m = ChessMove::new(C5, D6, ChessMoveType::EnPassant);
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(C5), None);
    //    assert_eq!(board.piece_at(D5), None);
    //    assert_eq!(board.piece_at(D6).unwrap(), (Piece::Pawn, Color::White));
    //
    //    let m = ChessMove::new(D6, D7, ChessMoveType::Quiet);
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(D6), None);
    //    assert_eq!(board.piece_at(D7).unwrap(), (Piece::Pawn, Color::White));
    //
    //    let m = ChessMove::new(D7, C8, ChessMoveType::QueenPromotionCapture);
    //    assert_eq!(
    //        board.piece_at(C8).unwrap(),
    //        (Piece::Bishop, Color::Black)
    //    );
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(D7), None);
    //    assert_eq!(
    //        board.piece_at(C8).unwrap(),
    //        (Piece::Queen, Color::White)
    //    );
    //
    //    let m = ChessMove::new(C8, D8, ChessMoveType::Capture);
    //    assert_eq!(
    //        board.piece_at(D8).unwrap(),
    //        (Piece::Queen, Color::Black)
    //    );
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(C8), None);
    //    assert_eq!(
    //        board.piece_at(D8).unwrap(),
    //        (Piece::Queen, Color::White)
    //    );
    //
    //    let m = ChessMove::new(D8, B8, ChessMoveType::Capture);
    //    assert_eq!(
    //        board.piece_at(B8).unwrap(),
    //        (Piece::Knight, Color::Black)
    //    );
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(D8), None);
    //    assert_eq!(
    //        board.piece_at(B8).unwrap(),
    //        (Piece::Queen, Color::White)
    //    );
    //
    //    let m = ChessMove::new(B8, B7, ChessMoveType::Capture);
    //    assert_eq!(board.piece_at(B7).unwrap(), (Piece::Pawn, Color::Black));
    //    let board = board.play_move(m, Color::White);
    //    assert_eq!(board.piece_at(B8), None);
    //    assert_eq!(
    //        board.piece_at(B7).unwrap(),
    //        (Piece::Queen, Color::White)
    //    );
    //
    //    let m = ChessMove::new(0, 0, ChessMoveType::QueenCastle);
    //    let board = board.play_move(m, Color::Black);
    //    assert_eq!(board.piece_at(A8), None);
    //    assert_eq!(board.piece_at(E8), None);
    //    assert_eq!(board.piece_at(C8).unwrap(), (Piece::King, Color::Black));
    //    assert_eq!(board.piece_at(D8).unwrap(), (Piece::Rook, Color::Black));
    //}
}
