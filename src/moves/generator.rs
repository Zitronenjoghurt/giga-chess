use crate::core::bitboard::BitBoard;
use crate::core::position::Position;
use crate::core::square::*;
use crate::moves::generator::table::AttackTable;
use crate::moves::list::MoveList;
use crate::prelude::{ChessBoard, ChessMove, Color, MoveFlags, Piece};
use std::sync::LazyLock;

mod magic;
mod table;

static MOVE_GEN: LazyLock<MoveGenerator> = LazyLock::new(MoveGenerator::build);

pub struct MoveGenerator {
    table: AttackTable,
}

impl MoveGenerator {
    pub fn get() -> &'static Self {
        &MOVE_GEN
    }

    pub(crate) fn build() -> Self {
        Self {
            table: AttackTable::build(),
        }
    }

    pub fn generate(&self, pos: &Position) -> MoveList {
        let pseudo_legal = self.generate_pseudo_legal(pos);

        let mut legal = MoveList::new();
        for mv in pseudo_legal {
            if self.is_legal(pos, mv) {
                legal.push(mv)
            }
        }

        legal
    }
}

// Generation
impl MoveGenerator {
    fn generate_pseudo_legal(&self, pos: &Position) -> MoveList {
        let player = pos.board.color_bb(pos.side_to_move);
        let opponent = pos.board.color_bb(pos.side_to_move.opposite());
        let occupied = player | opponent;

        let mut moves = MoveList::new();
        self.generate_pawn_moves(&mut moves, pos, opponent, occupied);
        self.generate_piece_moves(&mut moves, pos, Piece::Knight, player, opponent);
        self.generate_piece_moves(&mut moves, pos, Piece::King, player, opponent);
        self.generate_sliding_moves(&mut moves, pos, Piece::Bishop, player, opponent, occupied);
        self.generate_sliding_moves(&mut moves, pos, Piece::Rook, player, opponent, occupied);
        self.generate_sliding_moves(&mut moves, pos, Piece::Queen, player, opponent, occupied);
        self.generate_castling(&mut moves, pos, occupied);
        moves
    }

    fn generate_pawn_moves(
        &self,
        moves: &mut MoveList,
        pos: &Position,
        opponent: BitBoard,
        occupied: BitBoard,
    ) {
        let bb = pos.board.piece_bb(Piece::Pawn, pos.side_to_move);
        for from in bb {
            let move_mask = self.table.pawn_mask(from, pos.side_to_move) & !occupied;
            if let Some(move_to) = move_mask.get_lowest_set() {
                if !move_to.is_any_promotion_square() {
                    moves.push(ChessMove::from_flags(from, move_to, MoveFlags::Quiet));

                    let dpp_to = from.double_pawn_push(pos.side_to_move);
                    if from.is_pawn_start(pos.side_to_move) && !occupied.is_set(dpp_to) {
                        moves.push(ChessMove::from_flags(
                            from,
                            dpp_to,
                            MoveFlags::DoublePawnPush,
                        ));
                    }
                } else {
                    moves.extend(&ChessMove::promotions(from, move_to, false))
                }
            }

            let attack_mask = self.table.pawn_attacks(from, pos.side_to_move);
            for attack_to in attack_mask {
                if Some(attack_to) == pos.en_passant_square {
                    moves.push(ChessMove::from_flags(from, attack_to, MoveFlags::EnPassant));
                } else if opponent.is_set(attack_to) {
                    if !attack_to.is_any_promotion_square() {
                        moves.push(ChessMove::from_flags(from, attack_to, MoveFlags::Capture));
                    } else {
                        moves.extend(&ChessMove::promotions(from, attack_to, true));
                    }
                }
            }
        }
    }

    fn generate_piece_moves(
        &self,
        moves: &mut MoveList,
        pos: &Position,
        piece: Piece,
        player: BitBoard,
        opponent: BitBoard,
    ) {
        let bb = pos.board.piece_bb(piece, pos.side_to_move);
        for from in bb {
            let attacks = match piece {
                Piece::Knight => self.table.knight_attacks(from),
                Piece::King => self.table.king_attacks(from),
                _ => unreachable!(),
            } & !player;
            self.push_moves(moves, from, attacks, opponent);
        }
    }

    fn generate_sliding_moves(
        &self,
        moves: &mut MoveList,
        pos: &Position,
        piece: Piece,
        player: BitBoard,
        opponent: BitBoard,
        occupied: BitBoard,
    ) {
        let bb = pos.board.piece_bb(piece, pos.side_to_move);
        for from in bb {
            let attacks = match piece {
                Piece::Bishop => self.table.bishop_attacks(from, occupied),
                Piece::Rook => self.table.rook_attacks(from, occupied),
                Piece::Queen => self.table.queen_attacks(from, occupied),
                _ => unreachable!(),
            } & !player;
            self.push_moves(moves, from, attacks, opponent);
        }
    }

    fn generate_castling(&self, moves: &mut MoveList, pos: &Position, occupied_mask: BitBoard) {
        match pos.side_to_move {
            Color::White => {
                if pos.castling_rights.white_king_side
                    && (occupied_mask.value() & 0x60) == 0
                    && !self.is_square_attacked(&pos.board, E1, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, F1, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, G1, pos.side_to_move.opposite())
                {
                    moves.push(ChessMove::from_flags(E1, G1, MoveFlags::KingCastle));
                }
                if pos.castling_rights.white_queen_side
                    && (occupied_mask.value() & 0xE) == 0
                    && !self.is_square_attacked(&pos.board, C1, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, D1, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, E1, pos.side_to_move.opposite())
                {
                    moves.push(ChessMove::from_flags(E1, C1, MoveFlags::QueenCastle))
                }
            }
            Color::Black => {
                if pos.castling_rights.black_king_side
                    && (occupied_mask.value() & 0x6000000000000000) == 0
                    && !self.is_square_attacked(&pos.board, E8, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, F8, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, G8, pos.side_to_move.opposite())
                {
                    moves.push(ChessMove::from_flags(E8, G8, MoveFlags::KingCastle));
                }
                if pos.castling_rights.black_queen_side
                    && (occupied_mask.value() & 0xE00000000000000) == 0
                    && !self.is_square_attacked(&pos.board, C8, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, D8, pos.side_to_move.opposite())
                    && !self.is_square_attacked(&pos.board, E8, pos.side_to_move.opposite())
                {
                    moves.push(ChessMove::from_flags(E8, C8, MoveFlags::QueenCastle));
                }
            }
        };
    }

    fn push_moves(
        &self,
        moves: &mut MoveList,
        from: Square,
        attacks: BitBoard,
        opponent: BitBoard,
    ) {
        for to in attacks {
            let flags = if opponent.is_set(to) {
                MoveFlags::Capture
            } else {
                MoveFlags::Quiet
            };
            moves.push(ChessMove::from_flags(from, to, flags));
        }
    }
}

// Legality
impl MoveGenerator {
    pub fn is_legal(&self, pos: &Position, mv: ChessMove) -> bool {
        let next = (*pos).make_move(mv);
        !self.is_in_check(&next, pos.side_to_move)
    }

    pub fn is_in_check(&self, pos: &Position, color: Color) -> bool {
        let king_bb = pos.board.piece_bb(Piece::King, color);
        let Some(king_sq) = king_bb.get_lowest_set() else {
            return false;
        };
        self.is_square_attacked(&pos.board, king_sq, color.opposite())
    }

    pub fn is_square_attacked(&self, board: &ChessBoard, square: Square, by: Color) -> bool {
        let occupied = board.occupied_bb();

        let pawns =
            self.table.pawn_attacks(square, by.opposite()) & board.piece_bb(Piece::Pawn, by);
        if !pawns.is_empty() {
            return true;
        }

        let knights = self.table.knight_attacks(square) & board.piece_bb(Piece::Knight, by);
        if !knights.is_empty() {
            return true;
        }

        let king = self.table.king_attacks(square) & board.piece_bb(Piece::King, by);
        if !king.is_empty() {
            return true;
        }

        let diag = board.piece_bb(Piece::Bishop, by) | board.piece_bb(Piece::Queen, by);
        let bishops = self.table.bishop_attacks(square, occupied) & diag;
        if !bishops.is_empty() {
            return true;
        }

        let ortho = board.piece_bb(Piece::Rook, by) | board.piece_bb(Piece::Queen, by);
        let rooks = self.table.rook_attacks(square, occupied) & ortho;
        if !rooks.is_empty() {
            return true;
        }

        false
    }
}
