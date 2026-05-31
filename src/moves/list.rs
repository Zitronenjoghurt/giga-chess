use crate::prelude::ChessMove;

#[derive(Debug, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "bitcode", derive(bitcode::Encode, bitcode::Decode))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MoveList {
    #[cfg_attr(feature = "serde", serde(with = "crate::big_array"))]
    moves: [ChessMove; 256],
    len: u8,
}

impl Default for MoveList {
    fn default() -> Self {
        Self {
            moves: [ChessMove::default(); 256],
            len: 0,
        }
    }
}

impl MoveList {
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, mv: ChessMove) {
        self.moves[self.len as usize] = mv;
        self.len += 1;
    }

    pub(crate) fn extend(&mut self, other: &[ChessMove]) {
        self.moves[self.len as usize..self.len as usize + other.len()].copy_from_slice(other);
        self.len += other.len() as u8;
    }

    pub fn contains(&self, mv: ChessMove) -> bool {
        self.moves.contains(&mv)
    }

    pub fn as_slice(&self) -> &[ChessMove] {
        &self.moves[..self.len as usize]
    }

    pub fn iter(&self) -> impl Iterator<Item = &ChessMove> {
        self.as_slice().iter()
    }

    pub fn len(&self) -> usize {
        self.len as usize
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl IntoIterator for MoveList {
    type Item = ChessMove;
    type IntoIter = std::iter::Take<std::array::IntoIter<ChessMove, 256>>;

    fn into_iter(self) -> Self::IntoIter {
        self.moves.into_iter().take(self.len as usize)
    }
}

#[cfg(feature = "bit-codec")]
mod codec {
    pub use super::*;
    use bit_codec::{BitDecode, BitEncode, BitReader, BitWriter};
    use std::io::{Read, Write};

    impl BitEncode for MoveList {
        fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> std::io::Result<()> {
            w.write(&self.len)?;
            for mv in &self.moves[..self.len as usize] {
                w.write(mv)?;
            }
            Ok(())
        }
    }

    impl BitDecode for MoveList {
        fn decode<R: Read>(r: &mut BitReader<R>) -> std::io::Result<Self> {
            let len: u8 = r.read()?;
            let mut moves = [ChessMove::default(); 256];
            for (i, mv) in moves.iter_mut().enumerate() {
                if i >= len as usize {
                    break;
                }
                *mv = r.read()?;
            }
            Ok(Self { moves, len })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::square::*;
        use crate::prelude::{MoveKind, Piece};

        #[test]
        fn test_roundtrip() {
            use bit_codec::{BitReader, BitWriter};

            let mut list = MoveList::default();
            list.push(ChessMove::new(Square::A2, Square::A4, MoveKind::Capture));
            list.push(ChessMove::new(Square::E7, Square::E5, MoveKind::CastleKing));
            list.push(ChessMove::new(
                Square::D1,
                Square::H5,
                MoveKind::Promotion {
                    piece: Piece::Queen,
                    capture: true,
                },
            ));

            let mut buf = Vec::new();
            let mut w = BitWriter::new(&mut buf);
            w.write(&list).unwrap();
            w.flush().unwrap();
            drop(w);

            let mut r = BitReader::new(buf.as_slice());
            let decoded: MoveList = r.read().unwrap();

            assert_eq!(decoded.len, list.len);
            for i in 0..list.len as usize {
                assert_eq!(decoded.moves[i], list.moves[i]);
            }
        }
    }
}
