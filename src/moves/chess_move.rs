use crate::core::position::Position;
use crate::moves::generator::MoveGenerator;
use crate::prelude::{Piece, Square};
use std::fmt::{Display, Formatter};

// https://www.chessprogramming.org/Encoding_Moves
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "bit-codec",
    derive(bit_codec::BitEncode, bit_codec::BitDecode)
)]
#[repr(transparent)]
pub struct ChessMove(u16);

impl ChessMove {
    pub fn new(from: Square, to: Square, kind: MoveKind) -> Self {
        let flags = kind.into();
        Self::from_flags(from, to, flags)
    }

    pub fn from_flags(from: Square, to: Square, flags: MoveFlags) -> Self {
        Self(
            ((from.index() as u16) << 10)
                | (((to.index() & 0b111111) as u16) << 4)
                | (flags as u16),
        )
    }

    pub fn promotions(from: Square, to: Square, capture: bool) -> [Self; 4] {
        [Piece::Knight, Piece::Bishop, Piece::Rook, Piece::Queen]
            .map(|piece| Self::new(from, to, MoveKind::Promotion { piece, capture }))
    }

    pub fn from(&self) -> Square {
        Square::new((self.0 >> 10) as u8)
    }

    pub fn to(&self) -> Square {
        Square::new(((self.0 & 0b00000011_11110000) >> 4) as u8)
    }

    pub fn flags(&self) -> MoveFlags {
        MoveFlags::from(self.0 as u8)
    }

    pub fn kind(&self) -> MoveKind {
        self.flags().into()
    }

    pub fn is_capture(&self) -> bool {
        (self.0 & 0b0100) != 0
    }

    pub fn is_promotion(&self) -> bool {
        (self.0 & 0b1000) != 0
    }

    pub fn from_position(
        position: &Position,
        from: Square,
        to: Square,
        promotion: Option<Piece>,
    ) -> Option<ChessMove> {
        MoveGenerator::get()
            .generate(position)
            .iter()
            .find(|mv| {
                mv.from() == from && mv.to() == to && mv.flags().promotion_piece() == promotion
            })
            .copied()
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.from(), self.to())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum MoveFlags {
    Quiet = 0,
    DoublePawnPush = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Capture = 4,
    EnPassant = 5,
    KnightPromotion = 8,
    BishopPromotion = 9,
    RookPromotion = 10,
    QueenPromotion = 11,
    KnightPromoCap = 12,
    BishopPromoCap = 13,
    RookPromoCap = 14,
    QueenPromoCap = 15,
}

impl MoveFlags {
    pub fn is_capture(&self) -> bool {
        ((*self as u8) & 0b0100) != 0
    }

    pub fn is_promotion(&self) -> bool {
        ((*self as u8) & 0b1000) != 0
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        match self {
            Self::KnightPromotion | Self::KnightPromoCap => Some(Piece::Knight),
            Self::BishopPromotion | Self::BishopPromoCap => Some(Piece::Bishop),
            Self::RookPromotion | Self::RookPromoCap => Some(Piece::Rook),
            Self::QueenPromotion | Self::QueenPromoCap => Some(Piece::Queen),
            _ => None,
        }
    }

    pub fn is_kingside_castle(&self) -> bool {
        matches!(self, Self::KingCastle)
    }

    pub fn is_queenside_castle(&self) -> bool {
        matches!(self, Self::QueenCastle)
    }

    pub fn is_en_passant(&self) -> bool {
        matches!(self, Self::EnPassant)
    }
}

impl From<u8> for MoveFlags {
    fn from(value: u8) -> Self {
        match value & 0b1111 {
            0 => MoveFlags::Quiet,
            1 => MoveFlags::DoublePawnPush,
            2 => MoveFlags::KingCastle,
            3 => MoveFlags::QueenCastle,
            4 => MoveFlags::Capture,
            5 => MoveFlags::EnPassant,
            8 => MoveFlags::KnightPromotion,
            9 => MoveFlags::BishopPromotion,
            10 => MoveFlags::RookPromotion,
            11 => MoveFlags::QueenPromotion,
            12 => MoveFlags::KnightPromoCap,
            13 => MoveFlags::BishopPromoCap,
            14 => MoveFlags::RookPromoCap,
            15 => MoveFlags::QueenPromoCap,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum MoveKind {
    Quiet,
    DoublePawnPush,
    CastleKing,
    CastleQueen,
    Capture,
    EnPassant,
    Promotion { piece: Piece, capture: bool },
}

impl From<MoveFlags> for MoveKind {
    fn from(value: MoveFlags) -> Self {
        match value {
            MoveFlags::Quiet => MoveKind::Quiet,
            MoveFlags::DoublePawnPush => MoveKind::DoublePawnPush,
            MoveFlags::KingCastle => MoveKind::CastleKing,
            MoveFlags::QueenCastle => MoveKind::CastleQueen,
            MoveFlags::Capture => MoveKind::Capture,
            MoveFlags::EnPassant => MoveKind::EnPassant,
            MoveFlags::KnightPromotion => MoveKind::Promotion {
                piece: Piece::Knight,
                capture: false,
            },
            MoveFlags::BishopPromotion => MoveKind::Promotion {
                piece: Piece::Bishop,
                capture: false,
            },
            MoveFlags::RookPromotion => MoveKind::Promotion {
                piece: Piece::Rook,
                capture: false,
            },
            MoveFlags::QueenPromotion => MoveKind::Promotion {
                piece: Piece::Queen,
                capture: false,
            },
            MoveFlags::KnightPromoCap => MoveKind::Promotion {
                piece: Piece::Knight,
                capture: true,
            },
            MoveFlags::BishopPromoCap => MoveKind::Promotion {
                piece: Piece::Bishop,
                capture: true,
            },
            MoveFlags::RookPromoCap => MoveKind::Promotion {
                piece: Piece::Rook,
                capture: true,
            },
            MoveFlags::QueenPromoCap => MoveKind::Promotion {
                piece: Piece::Queen,
                capture: true,
            },
        }
    }
}

impl From<MoveKind> for MoveFlags {
    fn from(value: MoveKind) -> Self {
        match value {
            MoveKind::Quiet => MoveFlags::Quiet,
            MoveKind::DoublePawnPush => MoveFlags::DoublePawnPush,
            MoveKind::CastleKing => MoveFlags::KingCastle,
            MoveKind::CastleQueen => MoveFlags::QueenCastle,
            MoveKind::Capture => MoveFlags::Capture,
            MoveKind::EnPassant => MoveFlags::EnPassant,
            MoveKind::Promotion { piece, capture } => match (piece, capture) {
                (Piece::Knight, false) => MoveFlags::KnightPromotion,
                (Piece::Bishop, false) => MoveFlags::BishopPromotion,
                (Piece::Rook, false) => MoveFlags::RookPromotion,
                (Piece::Queen, false) => MoveFlags::QueenPromotion,
                (Piece::Knight, true) => MoveFlags::KnightPromoCap,
                (Piece::Bishop, true) => MoveFlags::BishopPromoCap,
                (Piece::Rook, true) => MoveFlags::RookPromoCap,
                (Piece::Queen, true) => MoveFlags::QueenPromoCap,
                _ => unreachable!(),
            },
        }
    }
}
