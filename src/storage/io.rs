use std::io::{self, Read, Write};

pub struct BitReader<'a> {
    bytes: &'a [u8],
    byte_pos: usize,
    bit_offset: u8,
}

impl<'a> BitReader<'a> {
    #[inline]
    pub fn new(bytes: &'a [u8]) -> Self {
        Self {
            bytes,
            byte_pos: 0,
            bit_offset: 0,
        }
    }

    #[inline]
    pub fn read<T: BitDecode>(&mut self) -> io::Result<T> {
        T::decode(self)
    }

    #[inline]
    pub fn read_bits(&mut self, count: u8) -> io::Result<u8> {
        if !(1..=8).contains(&count) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "bit count must be between 1 and 8",
            ));
        }

        if self.byte_pos >= self.bytes.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of input",
            ));
        }

        let bits_left = 8 - self.bit_offset;
        let to_read = count.min(bits_left);
        let shift = bits_left - to_read;
        let mask = ((1u16 << to_read) - 1) as u8;
        let bits = (self.bytes[self.byte_pos] >> shift) & mask;

        self.bit_offset += to_read;
        if self.bit_offset >= 8 {
            self.bit_offset = 0;
            self.byte_pos += 1;
        }

        if to_read < count {
            let rest = count - to_read;
            let next = self.read_bits(rest)?;
            Ok((bits << rest) | next)
        } else {
            Ok(bits)
        }
    }

    #[inline]
    pub fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_bits(1)? != 0)
    }

    #[inline]
    pub fn read_u8(&mut self) -> io::Result<u8> {
        self.read_bits(8)
    }

    pub fn read_full_bytes(&mut self, buf: &mut [u8]) -> io::Result<()> {
        if self.is_aligned() {
            let end = self.byte_pos + buf.len();
            if end > self.bytes.len() {
                return Err(io::Error::new(
                    io::ErrorKind::UnexpectedEof,
                    "Unexpected end of input",
                ));
            }
            buf.copy_from_slice(&self.bytes[self.byte_pos..end]);
            self.byte_pos = end;
        } else {
            for byte in buf.iter_mut() {
                *byte = self.read_u8()?;
            }
        }
        Ok(())
    }

    #[inline]
    pub fn position(&self) -> usize {
        self.byte_pos * 8 + self.bit_offset as usize
    }

    #[inline]
    pub fn remaining_bits(&self) -> usize {
        (self.bytes.len() - self.byte_pos) * 8 - self.bit_offset as usize
    }

    #[inline]
    pub fn is_aligned(&self) -> bool {
        self.bit_offset == 0
    }

    #[inline]
    pub fn align(&mut self) {
        if self.bit_offset != 0 {
            self.bit_offset = 0;
            self.byte_pos += 1;
        }
    }

    pub fn skip(&mut self, count: usize) -> io::Result<()> {
        if count > self.remaining_bits() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "Unexpected end of input",
            ));
        }
        let total = self.bit_offset as usize + count;
        self.byte_pos += total / 8;
        self.bit_offset = (total % 8) as u8;
        Ok(())
    }
}

impl<'a> Read for BitReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.is_aligned() {
            return Ok(0);
        }
        let avail = self.bytes.len() - self.byte_pos;
        let n = buf.len().min(avail);
        buf[..n].copy_from_slice(&self.bytes[self.byte_pos..self.byte_pos + n]);
        self.byte_pos += n;
        Ok(n)
    }
}

pub struct BitWriter<W: Write> {
    inner: W,
    current_byte: u8,
    bit_count: u8,
}

impl<W: Write> BitWriter<W> {
    #[inline]
    pub fn new(inner: W) -> Self {
        Self {
            inner,
            current_byte: 0,
            bit_count: 0,
        }
    }

    #[inline]
    pub fn write<T: BitEncode>(&mut self, v: &T) -> io::Result<()> {
        v.encode(self)
    }

    #[inline]
    pub fn write_bits(&mut self, value: u8, count: u8) -> io::Result<()> {
        if !(1..=8).contains(&count) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "bit count must be between 1 and 8",
            ));
        }

        let space = 8 - self.bit_count;
        let to_write = count.min(space);
        let remaining = count - to_write;

        let bits = if remaining > 0 {
            value >> remaining
        } else {
            value
        };
        let mask = ((1u16 << to_write) - 1) as u8;
        self.current_byte |= (bits & mask) << (space - to_write);
        self.bit_count += to_write;

        if self.bit_count == 8 {
            self.inner.write_all(&[self.current_byte])?;
            self.current_byte = 0;
            self.bit_count = 0;
        }

        if remaining > 0 {
            self.write_bits(value, remaining)?;
        }

        Ok(())
    }

    #[inline]
    pub fn write_bool(&mut self, v: bool) -> io::Result<()> {
        self.write_bits(v as u8, 1)
    }

    #[inline]
    pub fn write_u8(&mut self, v: u8) -> io::Result<()> {
        self.write_bits(v, 8)
    }

    pub fn write_full_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.is_aligned() {
            self.inner.write_all(bytes)?;
        } else {
            for &b in bytes {
                self.write_u8(b)?;
            }
        }
        Ok(())
    }

    #[inline]
    pub fn is_aligned(&self) -> bool {
        self.bit_count == 0
    }

    /// Pad remaining bits with zeros to reach a byte boundary.
    pub fn align(&mut self) -> io::Result<()> {
        if self.bit_count > 0 {
            self.inner.write_all(&[self.current_byte])?;
            self.current_byte = 0;
            self.bit_count = 0;
        }
        Ok(())
    }

    /// Pad to byte boundary and flush the underlying writer.
    /// Always call this when done writing.
    pub fn flush(&mut self) -> io::Result<()> {
        self.align()?;
        self.inner.flush()
    }

    /// Consume the writer. Does NOT auto-flush — call `.flush()` first.
    pub fn into_inner(self) -> W {
        self.inner
    }
}

impl<W: Write> Write for BitWriter<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if !self.is_aligned() {
            return Ok(0);
        }
        self.inner.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        BitWriter::flush(self)
    }
}

pub trait BitEncode {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()>;
}

pub trait BitDecode: Sized {
    fn decode(r: &mut BitReader) -> io::Result<Self>;
}

impl BitEncode for u8 {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        w.write_u8(*self)
    }
}

impl BitDecode for u8 {
    fn decode(r: &mut BitReader) -> io::Result<Self> {
        r.read_u8()
    }
}

macro_rules! impl_bit_codec {
    ($($ty:ty),*) => {
        $(
            impl BitEncode for $ty {
                fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
                    w.write_full_bytes(&self.to_le_bytes())
                }
            }

            impl BitDecode for $ty {
                fn decode(r: &mut BitReader) -> io::Result<Self> {
                    let mut buf = [0u8; std::mem::size_of::<$ty>()];
                    r.read_full_bytes(&mut buf)?;
                    Ok(<$ty>::from_le_bytes(buf))
                }
            }
        )*
    };
}

impl_bit_codec!(u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

impl BitEncode for bool {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        w.write_bool(*self)
    }
}

impl BitDecode for bool {
    fn decode(r: &mut BitReader) -> io::Result<Self> {
        r.read_bool()
    }
}

impl BitEncode for String {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        (self.len() as u32).encode(w)?;
        w.write_full_bytes(self.as_bytes())
    }
}

impl BitDecode for String {
    fn decode(r: &mut BitReader) -> io::Result<Self> {
        let len = u32::decode(r)? as usize;
        let mut buf = vec![0u8; len];
        r.read_full_bytes(&mut buf)?;
        String::from_utf8(buf).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}

impl BitEncode for &str {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        (self.len() as u32).encode(w)?;
        w.write_full_bytes(self.as_bytes())
    }
}

impl<T: BitEncode> BitEncode for Vec<T> {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        (self.len() as u32).encode(w)?;
        for item in self {
            item.encode(w)?;
        }
        Ok(())
    }
}

impl<T: BitDecode> BitDecode for Vec<T> {
    fn decode(r: &mut BitReader) -> io::Result<Self> {
        let len = u32::decode(r)? as usize;
        (0..len).map(|_| T::decode(r)).collect()
    }
}

impl<T: BitEncode> BitEncode for &[T] {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        (self.len() as u32).encode(w)?;
        for item in *self {
            item.encode(w)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_integers() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write(&42u8).unwrap();
            w.write(&0xABCDu16).unwrap();
            w.write(&0xDEAD_BEEFu32).unwrap();
            w.write(&0x0102_0304_0506_0708u64).unwrap();
            w.write(&-1i32).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read::<u8>().unwrap(), 42);
        assert_eq!(r.read::<u16>().unwrap(), 0xABCD);
        assert_eq!(r.read::<u32>().unwrap(), 0xDEAD_BEEF);
        assert_eq!(r.read::<u64>().unwrap(), 0x0102_0304_0506_0708);
        assert_eq!(r.read::<i32>().unwrap(), -1);
    }

    #[test]
    fn round_trip_bool() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write(&true).unwrap();
            w.write(&false).unwrap();
            w.write(&true).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert!(r.read::<bool>().unwrap());
        assert!(r.read::<bool>().unwrap());
        assert!(r.read::<bool>().unwrap());
    }

    #[test]
    fn round_trip_string() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write(&String::from("hello")).unwrap();
            w.write(&"world").unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read::<String>().unwrap(), "hello");
        assert_eq!(r.read::<String>().unwrap(), "world");
    }

    #[test]
    fn round_trip_vec() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write(&vec![1u32, 2, 3, 4, 5]).unwrap();
            w.write(&vec!["foo".to_string(), "bar".to_string()])
                .unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read::<Vec<u32>>().unwrap(), vec![1, 2, 3, 4, 5]);
        assert_eq!(
            r.read::<Vec<String>>().unwrap(),
            vec!["foo".to_string(), "bar".to_string()]
        );
    }

    #[test]
    fn round_trip_mixed_bits() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(0b110, 3).unwrap();
            w.write(&true).unwrap();
            w.write(&0xFFu8).unwrap();
            w.write(&1024u16).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read_bits(3).unwrap(), 0b110);
        assert!(r.read::<bool>().unwrap());
        assert_eq!(r.read::<u8>().unwrap(), 0xFF);
        assert_eq!(r.read::<u16>().unwrap(), 1024);
    }

    #[test]
    fn nibbles() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(0xA, 4).unwrap();
            w.write_bits(0x5, 4).unwrap();
            w.flush().unwrap();
        }
        assert_eq!(buf, [0xA5]);

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read_bits(4).unwrap(), 0xA);
        assert_eq!(r.read_bits(4).unwrap(), 0x5);
    }

    #[test]
    fn byte_level() {
        let original = b"Hello, bits!";
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_full_bytes(original).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        let mut out = vec![0u8; original.len()];
        r.read_full_bytes(&mut out).unwrap();
        assert_eq!(&out, original);
    }

    #[test]
    fn alignment() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(0b111, 3).unwrap();
            w.align().unwrap();
            w.write(&0xABu8).unwrap();
            w.flush().unwrap();
        }
        assert_eq!(buf, [0b1110_0000, 0xAB]);
    }

    #[test]
    fn position_and_remaining() {
        let data = [0xFF, 0xFF];
        let mut r = BitReader::new(&data);
        assert_eq!(r.position(), 0);
        assert_eq!(r.remaining_bits(), 16);
        r.read_bits(3).unwrap();
        assert_eq!(r.position(), 3);
        assert_eq!(r.remaining_bits(), 13);
    }

    #[test]
    fn skip_bits() {
        let data = [0b1111_0000, 0b1010_1010];
        let mut r = BitReader::new(&data);
        r.skip(4).unwrap();
        assert_eq!(r.read_bits(4).unwrap(), 0b0000);
        r.skip(4).unwrap();
        assert_eq!(r.read_bits(4).unwrap(), 0b1010);
    }

    #[test]
    fn std_read_trait() {
        let data = [0x01, 0x02, 0x03, 0x04];
        let mut r = BitReader::new(&data);
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf).unwrap();
        assert_eq!(buf, data);
    }

    #[test]
    fn std_write_trait() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_all(b"raw bytes").unwrap();
            w.flush().unwrap();
        }
        assert_eq!(&buf, b"raw bytes");
    }

    #[test]
    fn eof_error() {
        let data = [0xFF];
        let mut r = BitReader::new(&data);
        r.read::<u8>().unwrap();
        assert!(r.read_bits(1).is_err());
    }

    #[test]
    fn round_trip_floats() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write(&std::f32::consts::PI).unwrap();
            w.write(&std::f64::consts::E).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read::<f32>().unwrap(), std::f32::consts::PI);
        assert_eq!(r.read::<f64>().unwrap(), std::f64::consts::E);
    }

    #[test]
    fn round_trip_slice() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            let data: &[u16] = &[10, 20, 30];
            w.write(&data).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(&buf);
        assert_eq!(r.read::<Vec<u16>>().unwrap(), vec![10, 20, 30]);
    }
}
