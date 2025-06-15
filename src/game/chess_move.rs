use crate::game::piece::Piece;
use crate::game::square::Square;
use std::fmt::{Display, Formatter};

// https://www.chessprogramming.org/Encoding_Moves
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bincode", derive(bincode::Encode, bincode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct ChessMove(u16);

impl ChessMove {
    pub fn new(from: u8, to: u8, move_type: ChessMoveType) -> Self {
        Self(((from as u16) << 10) | ((to as u16) << 4) | (move_type as u16))
    }

    pub fn all_promotions(from: u8, to: u8) -> Vec<Self> {
        vec![
            Self::new(from, to, ChessMoveType::KnightPromotion),
            Self::new(from, to, ChessMoveType::BishopPromotion),
            Self::new(from, to, ChessMoveType::RookPromotion),
            Self::new(from, to, ChessMoveType::QueenPromotion),
        ]
    }

    pub fn all_promotions_capture(from: u8, to: u8) -> Vec<Self> {
        vec![
            Self::new(from, to, ChessMoveType::KnightPromotionCapture),
            Self::new(from, to, ChessMoveType::BishopPromotionCapture),
            Self::new(from, to, ChessMoveType::RookPromotionCapture),
            Self::new(from, to, ChessMoveType::QueenPromotionCapture),
        ]
    }

    pub fn get_from(&self) -> u8 {
        (self.0 >> 10) as u8
    }

    pub fn get_to(&self) -> u8 {
        ((self.0 & 0b00000011_11110000) >> 4) as u8
    }

    pub fn get_type(&self) -> ChessMoveType {
        ChessMoveType::from(self.0 as u8)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ChessMoveType {
    Quiet = 0,
    DoublePawnPush = 1,
    KingCastle = 2,
    QueenCastle = 3,
    Capture = 4,
    EnPassant = 5,
    Reserved6 = 6,
    Reserved7 = 7,
    KnightPromotion = 8,
    BishopPromotion = 9,
    RookPromotion = 10,
    QueenPromotion = 11,
    KnightPromotionCapture = 12,
    BishopPromotionCapture = 13,
    RookPromotionCapture = 14,
    QueenPromotionCapture = 15,
}

impl ChessMoveType {
    pub fn is_capture(&self) -> bool {
        (((*self as u8) >> 2) & 1) == 1
    }

    pub fn is_promotion(&self) -> bool {
        (*self as u8) >= 8
    }

    pub fn promotion_piece(&self) -> Option<Piece> {
        match self {
            Self::KnightPromotion | Self::KnightPromotionCapture => Some(Piece::Knight),
            Self::BishopPromotion | Self::BishopPromotionCapture => Some(Piece::Bishop),
            Self::RookPromotion | Self::RookPromotionCapture => Some(Piece::Rook),
            Self::QueenPromotion | Self::QueenPromotionCapture => Some(Piece::Queen),
            _ => None,
        }
    }
}

impl From<u8> for ChessMoveType {
    fn from(value: u8) -> Self {
        match value & 0b1111 {
            0 => ChessMoveType::Quiet,
            1 => ChessMoveType::DoublePawnPush,
            2 => ChessMoveType::KingCastle,
            3 => ChessMoveType::QueenCastle,
            4 => ChessMoveType::Capture,
            5 => ChessMoveType::EnPassant,
            6 => ChessMoveType::Reserved6,
            7 => ChessMoveType::Reserved7,
            8 => ChessMoveType::KnightPromotion,
            9 => ChessMoveType::BishopPromotion,
            10 => ChessMoveType::RookPromotion,
            11 => ChessMoveType::QueenPromotion,
            12 => ChessMoveType::KnightPromotionCapture,
            13 => ChessMoveType::BishopPromotionCapture,
            14 => ChessMoveType::RookPromotionCapture,
            15 => ChessMoveType::QueenPromotionCapture,
            _ => unreachable!(),
        }
    }
}

impl Display for ChessMove {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let from = Square::new(self.get_from()).to_string().to_lowercase();
        let to = Square::new(self.get_to()).to_string().to_lowercase();

        match self.get_type() {
            ChessMoveType::Quiet => write!(f, "{}-{}", from, to),
            ChessMoveType::DoublePawnPush => write!(f, "{} DPP", from),
            ChessMoveType::KingCastle => write!(f, "KC"),
            ChessMoveType::QueenCastle => write!(f, "QC"),
            ChessMoveType::Capture => write!(f, "{}-{} C", from, to),
            ChessMoveType::EnPassant => write!(f, "{}-{} EP", from, to),
            ChessMoveType::Reserved6 => write!(f, "R6"),
            ChessMoveType::Reserved7 => write!(f, "R7"),
            ChessMoveType::KnightPromotion => write!(f, "{}-{} PN", from, to),
            ChessMoveType::BishopPromotion => write!(f, "{}-{} PB", from, to),
            ChessMoveType::RookPromotion => write!(f, "{}-{} PR", from, to),
            ChessMoveType::QueenPromotion => write!(f, "{}-{} PQ", from, to),
            ChessMoveType::KnightPromotionCapture => write!(f, "{}-{} PN C", from, to),
            ChessMoveType::BishopPromotionCapture => write!(f, "{}-{} PB C", from, to),
            ChessMoveType::RookPromotionCapture => write!(f, "{}-{} PR C", from, to),
            ChessMoveType::QueenPromotionCapture => write!(f, "{}-{} PQ C", from, to),
        }
    }
}
