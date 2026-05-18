use crate::core::bitboard::BitBoard;
use crate::core::position::Position;
use crate::moves::generator::table::AttackTable;
use crate::moves::list::MoveList;
use crate::prelude::{ChessMove, MoveFlags, Piece, Square};
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

    // ToDo: Implement
    //pub fn generate(&self, position: &Position) -> MoveList {}

    fn generate_pseudo_legal(&self, pos: &Position) -> MoveList {
        let player = pos.board.color_bb(pos.side_to_move);
        let opponent = pos.board.color_bb(pos.side_to_move.opposite());
        let occupied = player | opponent;

        let mut moves = MoveList::new();
        //self.generate_pawn_moves(&mut moves, pos, opponent, occupied);
        self.generate_piece_moves(&mut moves, pos, Piece::Knight, player, opponent);
        self.generate_piece_moves(&mut moves, pos, Piece::King, player, opponent);
        self.generate_sliding_moves(&mut moves, pos, Piece::Bishop, player, opponent, occupied);
        self.generate_sliding_moves(&mut moves, pos, Piece::Rook, player, opponent, occupied);
        self.generate_sliding_moves(&mut moves, pos, Piece::Queen, player, opponent, occupied);
        //self.generate_castling(&mut moves, pos, occupied);
        moves
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
