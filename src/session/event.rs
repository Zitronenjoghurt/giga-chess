use crate::game::outcome::GameOutcome;
use crate::prelude::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "bit-codec",
    derive(bit_codec::BitEncode, bit_codec::BitDecode)
)]
#[cfg_attr(feature = "bit-codec", bits(disc = 2))]
pub enum SessionEvent {
    BoardUpdate,
    DrawOffered { by: Color },
    DrawOfferDeclined { by: Color },
    GameOver(GameOutcome),
}
