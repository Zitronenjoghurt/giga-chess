use crate::prelude::{ChessMove, Piece, Square};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SessionAction {
    Move(ChessMove),
    MoveFromTo {
        from: Square,
        to: Square,
        promotion: Option<Piece>,
    },
    Resign,
    OfferDraw,
    AcceptDraw,
    DeclineDraw,
    ClaimDraw,
    ClaimTimeout,
}
