use crate::impl_bit_vec_header;
use crate::moves::naive::{NaiveMove, NaivePromotionMove};
use crate::storage::io::{BitDecode, BitEncode, BitReader, BitWriter};
use std::io::{Read, Write};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredNaiveMoves(Vec<u8>);

impl StoredNaiveMoves {
    pub fn new(moves: &[NaiveMove]) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            moves.encode(&mut writer)?;
            writer.flush()?;
        }
        Ok(Self(buffer))
    }

    pub fn restore(self) -> std::io::Result<Vec<NaiveMove>> {
        let mut reader = BitReader::new(self.0.as_slice());
        BitDecode::decode(&mut reader)
    }
}

impl BitEncode for NaiveMove {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        w.write(&self.from)?;
        w.write(&self.to)?;
        Ok(())
    }
}

impl BitDecode for NaiveMove {
    fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
        let from = r.read()?;
        let to = r.read()?;
        Ok(Self::new(from, to))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StoredNaivePromotionMoves(Vec<u8>);

impl StoredNaivePromotionMoves {
    pub fn new(moves: &[NaivePromotionMove]) -> std::io::Result<Self> {
        let mut buffer = Vec::new();
        {
            let mut writer = BitWriter::new(&mut buffer);
            moves.encode(&mut writer)?;
            writer.flush()?;
        }
        Ok(Self(buffer))
    }

    pub fn restore(self) -> std::io::Result<Vec<NaivePromotionMove>> {
        let mut reader = BitReader::new(self.0.as_slice());
        BitDecode::decode(&mut reader)
    }
}

impl BitEncode for NaivePromotionMove {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
        w.write(&self.mv)?;
        w.write(&self.promotion)?;
        Ok(())
    }
}

impl BitDecode for NaivePromotionMove {
    fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
        let mv = r.read()?;
        let promotion = r.read()?;
        Ok(Self { mv, promotion })
    }
}

impl_bit_vec_header!(u16, NaiveMove, NaivePromotionMove);
