use crate::engine::attack_table::AttackTable;
use crate::game::bit_board::BitBoard;
use crate::game::chess_board::ChessBoard;
use crate::game::chess_move::{ChessMove, ChessMoveType};
use crate::game::color::Color;
use crate::game::piece::Piece;
use crate::game::state::GameState;
use crate::game::status::GameStatus;
use std::sync::Arc;

pub mod attack_table;
pub mod magic_numbers;

pub struct Engine {
    pub attack_table: AttackTable,
}

// ToDo: Check for insufficient material, 3-fold repetition
impl Engine {
    pub fn initialize() -> Arc<Self> {
        Arc::new(Self {
            attack_table: AttackTable::build(),
        })
    }

    pub fn generate_moves(&self, game_state: &GameState) -> (Vec<ChessMove>, GameStatus) {
        if game_state.half_moves >= 50 {
            return (vec![], GameStatus::DrawFiftyMove);
        }

        let player_mask = game_state.board.get_color_bb(game_state.side_to_move);
        let opponent_mask = game_state
            .board
            .get_color_bb(game_state.side_to_move.opposite());
        let occupied_mask = player_mask | opponent_mask;

        let pseudo_legal =
            self.generate_pseudo_legal_moves(game_state, player_mask, opponent_mask, occupied_mask);

        let legal = pseudo_legal
            .into_iter()
            .filter(|&mv| self.is_legal_move(game_state, mv))
            .collect::<Vec<_>>();
        if legal.is_empty() {
            return if self.is_in_check(game_state.board, game_state.side_to_move) {
                (legal, GameStatus::Checkmate)
            } else {
                (legal, GameStatus::Stalemate)
            };
        }

        (legal, GameStatus::Running)
    }

    pub fn is_legal_move(&self, game_state: &GameState, chess_move: ChessMove) -> bool {
        let future_board = game_state
            .board
            .play_move(chess_move, game_state.side_to_move);
        !self.is_in_check(future_board, game_state.side_to_move)
    }

    pub fn is_in_check(&self, board: ChessBoard, color: Color) -> bool {
        let king_bb = board.get_piece_bb(Piece::King, color);
        let Some(king_square) = king_bb.get_lowest_set_bit() else {
            return false;
        };

        let opponent_color = color.opposite();
        self.is_square_attacked(board, king_square, opponent_color)
    }

    pub fn is_square_attacked(&self, board: ChessBoard, square: u8, opponent_color: Color) -> bool {
        let occupied = board.get_occupied_bb();

        let pawn_attacks = self
            .attack_table
            .get_pawn_king_attack(square, opponent_color);
        if !(pawn_attacks & board.get_piece_bb(Piece::Pawn, opponent_color)).is_empty() {
            return true;
        }

        let knight_attacks = self.attack_table.get_knight_attacks(square);
        if !(knight_attacks & board.get_piece_bb(Piece::Knight, opponent_color)).is_empty() {
            return true;
        }

        let bishop_attacks = self.attack_table.get_bishop_attacks(square, occupied);
        if !(bishop_attacks
            & (board.get_piece_bb(Piece::Bishop, opponent_color)
                | board.get_piece_bb(Piece::Queen, opponent_color)))
        .is_empty()
        {
            return true;
        }

        let rook_attacks = self.attack_table.get_rook_attacks(square, occupied);
        if !(rook_attacks
            & (board.get_piece_bb(Piece::Rook, opponent_color)
                | board.get_piece_bb(Piece::Queen, opponent_color)))
        .is_empty()
        {
            return true;
        }

        let king_attacks = self.attack_table.get_king_attacks(square);
        if !(king_attacks & board.get_piece_bb(Piece::King, opponent_color)).is_empty() {
            return true;
        }

        false
    }

    pub fn is_promotion(&self, game_state: &GameState, move_to: u8) -> bool {
        game_state.side_to_move == Color::White && move_to > 55
            || game_state.side_to_move == Color::Black && move_to < 8
    }

    pub fn get_pawn_double_push_target(
        &self,
        game_state: &GameState,
        from: u8,
        occupied_mask: BitBoard,
    ) -> Option<u8> {
        if game_state.side_to_move == Color::White && from > 7 && from < 16
            || game_state.side_to_move == Color::Black && from > 47 && from < 56
        {
            let target_square = match game_state.side_to_move {
                Color::White => from + 16,
                Color::Black => from - 16,
            };
            if !occupied_mask.get_bit(target_square) {
                Some(target_square)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn generate_pseudo_legal_moves(
        &self,
        game_state: &GameState,
        player_mask: BitBoard,
        opponent_mask: BitBoard,
        occupied_mask: BitBoard,
    ) -> Vec<ChessMove> {
        let mut moves = Vec::new();
        self.generate_pawn_moves(&mut moves, game_state, opponent_mask, occupied_mask);
        self.generate_knight_moves(&mut moves, game_state, player_mask, opponent_mask);
        self.generate_bishop_moves(
            &mut moves,
            game_state,
            player_mask,
            opponent_mask,
            occupied_mask,
        );
        self.generate_rook_moves(
            &mut moves,
            game_state,
            player_mask,
            opponent_mask,
            occupied_mask,
        );
        self.generate_king_moves(&mut moves, game_state, player_mask, opponent_mask);
        self.generate_queen_moves(
            &mut moves,
            game_state,
            player_mask,
            opponent_mask,
            occupied_mask,
        );
        self.generate_possible_castling_moves(&mut moves, game_state, occupied_mask);
        moves
    }

    fn generate_pawn_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        opponent_mask: BitBoard,
        occupied_mask: BitBoard,
    ) {
        let pawn_bb = game_state
            .board
            .get_piece_bb(Piece::Pawn, game_state.side_to_move);

        for from in pawn_bb.iter_set_bits() {
            let mask = self
                .attack_table
                .get_pawn_mask(from, game_state.side_to_move)
                & !occupied_mask;
            let attack = self
                .attack_table
                .get_pawn_attacks(from, game_state.side_to_move);

            if let Some(move_to) = mask.get_lowest_set_bit() {
                if !self.is_promotion(game_state, move_to) {
                    moves.push(ChessMove::new(from, move_to, ChessMoveType::Quiet));

                    if let Some(double_push_target) =
                        self.get_pawn_double_push_target(game_state, from, occupied_mask)
                    {
                        moves.push(ChessMove::new(
                            from,
                            double_push_target,
                            ChessMoveType::DoublePawnPush,
                        ));
                    }
                } else {
                    moves.extend(ChessMove::all_promotions(from, move_to))
                }
            }

            for move_to in attack.iter_set_bits() {
                if Some(move_to) == game_state.en_passant_square {
                    moves.push(ChessMove::new(from, move_to, ChessMoveType::EnPassant));
                } else if opponent_mask.get_bit(move_to) {
                    if !self.is_promotion(game_state, move_to) {
                        moves.push(ChessMove::new(from, move_to, ChessMoveType::Capture));
                    } else {
                        moves.extend(ChessMove::all_promotions_capture(from, move_to))
                    }
                }
            }
        }
    }

    fn generate_knight_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        player_mask: BitBoard,
        opponent_mask: BitBoard,
    ) {
        let knight_bb = game_state
            .board
            .get_piece_bb(Piece::Knight, game_state.side_to_move);
        for from in knight_bb.iter_set_bits() {
            let attack = self.attack_table.get_knight_attacks(from) & !player_mask;
            self.push_quiet_or_capture_moves(moves, from, attack, opponent_mask);
        }
    }

    fn generate_bishop_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        player_mask: BitBoard,
        opponent_mask: BitBoard,
        occupied_mask: BitBoard,
    ) {
        let bishop_bb = game_state
            .board
            .get_piece_bb(Piece::Bishop, game_state.side_to_move);
        for from in bishop_bb.iter_set_bits() {
            let attack = self.attack_table.get_bishop_attacks(from, occupied_mask) & !player_mask;
            self.push_quiet_or_capture_moves(moves, from, attack, opponent_mask);
        }
    }

    fn generate_rook_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        player_mask: BitBoard,
        opponent_mask: BitBoard,
        occupied_mask: BitBoard,
    ) {
        let rook_bb = game_state
            .board
            .get_piece_bb(Piece::Rook, game_state.side_to_move);
        for from in rook_bb.iter_set_bits() {
            let attack = self.attack_table.get_rook_attacks(from, occupied_mask) & !player_mask;
            self.push_quiet_or_capture_moves(moves, from, attack, opponent_mask);
        }
    }

    fn generate_king_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        player_mask: BitBoard,
        opponent_mask: BitBoard,
    ) {
        let king_bb = game_state
            .board
            .get_piece_bb(Piece::King, game_state.side_to_move);
        for from in king_bb.iter_set_bits() {
            let attack = self.attack_table.get_king_attacks(from) & !player_mask;
            self.push_quiet_or_capture_moves(moves, from, attack, opponent_mask);
        }
    }

    fn generate_queen_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        player_mask: BitBoard,
        opponent_mask: BitBoard,
        occupied_mask: BitBoard,
    ) {
        let queen_bb = game_state
            .board
            .get_piece_bb(Piece::Queen, game_state.side_to_move);
        for from in queen_bb.iter_set_bits() {
            let attack = self.attack_table.get_queen_attacks(from, occupied_mask) & !player_mask;
            self.push_quiet_or_capture_moves(moves, from, attack, opponent_mask);
        }
    }

    fn generate_possible_castling_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        game_state: &GameState,
        occupied_mask: BitBoard,
    ) {
        match game_state.side_to_move {
            Color::White => {
                if game_state.castling_rights.white_king_side
                    && (occupied_mask.get_value() & 0x60) == 0
                    && !self.is_square_attacked(
                        game_state.board,
                        4,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        5,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        6,
                        game_state.side_to_move.opposite(),
                    )
                {
                    moves.push(ChessMove::new(4, 6, ChessMoveType::KingCastle));
                }
                if game_state.castling_rights.white_queen_side
                    && (occupied_mask.get_value() & 0xE) == 0
                    && !self.is_square_attacked(
                        game_state.board,
                        2,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        3,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        4,
                        game_state.side_to_move.opposite(),
                    )
                {
                    moves.push(ChessMove::new(4, 2, ChessMoveType::QueenCastle))
                }
            }
            Color::Black => {
                if game_state.castling_rights.black_king_side
                    && (occupied_mask.get_value() & 0x6000000000000000) == 0
                    && !self.is_square_attacked(
                        game_state.board,
                        60,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        61,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        62,
                        game_state.side_to_move.opposite(),
                    )
                {
                    moves.push(ChessMove::new(60, 62, ChessMoveType::KingCastle));
                }
                if game_state.castling_rights.black_queen_side
                    && (occupied_mask.get_value() & 0xE00000000000000) == 0
                    && !self.is_square_attacked(
                        game_state.board,
                        58,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        59,
                        game_state.side_to_move.opposite(),
                    )
                    && !self.is_square_attacked(
                        game_state.board,
                        60,
                        game_state.side_to_move.opposite(),
                    )
                {
                    moves.push(ChessMove::new(60, 58, ChessMoveType::QueenCastle));
                }
            }
        };
    }

    fn push_quiet_or_capture_moves(
        &self,
        moves: &mut Vec<ChessMove>,
        from: u8,
        attack_mask: BitBoard,
        opponent_mask: BitBoard,
    ) {
        for move_to in attack_mask.iter_set_bits() {
            if opponent_mask.get_bit(move_to) {
                moves.push(ChessMove::new(from, move_to, ChessMoveType::Capture));
            } else {
                moves.push(ChessMove::new(from, move_to, ChessMoveType::Quiet));
            }
        }
    }
}
