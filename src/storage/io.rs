use std::io::{self, Read, Write};

pub struct BitReader<R: Read> {
    inner: R,
    buf: u8,
    bits_left: u8,
}

impl<R: Read> BitReader<R> {
    #[inline]
    pub fn new(inner: R) -> Self {
        Self {
            inner,
            buf: 0,
            bits_left: 0,
        }
    }

    #[inline]
    pub fn read<T: BitDecode>(&mut self) -> io::Result<T> {
        T::decode(self)
    }

    #[inline]
    pub fn read_bits<T: BitInt>(&mut self, count: u32) -> io::Result<T> {
        if count == 0 || count > T::BITS {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad bit count"));
        }
        let mut acc = T::ZERO;
        let mut remaining = count;
        while remaining > 0 {
            if self.bits_left == 0 {
                let mut b = [0u8];
                self.inner.read_exact(&mut b)?;
                self.buf = b[0];
                self.bits_left = 8;
            }
            let chunk = remaining.min(self.bits_left as u32);
            let lo_shift = self.bits_left as u32 - chunk;
            let mask: u8 = if chunk == 8 { 0xFF } else { (1u8 << chunk) - 1 };
            let bits = (self.buf >> lo_shift) & mask;
            remaining -= chunk;
            acc = acc.deposit_byte(bits, remaining);
            self.bits_left -= chunk as u8;
        }
        Ok(acc)
    }

    pub fn read_full_bytes(&mut self, out: &mut [u8]) -> io::Result<()> {
        if self.is_aligned() {
            self.inner.read_exact(out)
        } else {
            for byte in out.iter_mut() {
                *byte = self.read_bits(8)?;
            }
            Ok(())
        }
    }

    pub fn is_aligned(&self) -> bool {
        self.bits_left == 0
    }

    pub fn align(&mut self) {
        self.bits_left = 0;
    }
}

impl<R: Read> Read for BitReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if !self.is_aligned() {
            return Ok(0);
        }
        self.inner.read(buf)
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
    pub fn write<T: BitEncode + ?Sized>(&mut self, v: &T) -> io::Result<()> {
        v.encode(self)
    }

    #[inline]
    pub fn write_bits<T: BitInt>(&mut self, value: T, count: u32) -> io::Result<()> {
        if count == 0 || count > T::BITS {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad bit count"));
        }
        let mut remaining = count;
        while remaining > 0 {
            let space = 8 - self.bit_count as u32;
            let chunk = remaining.min(space);
            let shift = remaining - chunk;
            let mask: u8 = if chunk == 8 { 0xFF } else { (1u8 << chunk) - 1 };
            let bits = value.extract_byte(shift) & mask;
            self.current_byte |= bits << (space - chunk);
            self.bit_count += chunk as u8;
            if self.bit_count == 8 {
                self.inner.write_all(&[self.current_byte])?;
                self.current_byte = 0;
                self.bit_count = 0;
            }
            remaining -= chunk;
        }
        Ok(())
    }

    pub fn write_full_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        if self.is_aligned() {
            self.inner.write_all(bytes)?;
        } else {
            for &b in bytes {
                self.write_bits(b, 8)?;
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
    fn decode<R: Read>(r: &mut BitReader<R>) -> io::Result<Self>;
}

pub trait BitInt: Copy {
    const BITS: u32;
    const ZERO: Self;
    /// Low 8 bits of `self >> shift`. Caller guarantees `shift < Self::BITS`.
    fn extract_byte(self, shift: u32) -> u8;
    /// `self | ((bits as Self) << shift)`. Caller guarantees `shift < Self::BITS`.
    fn deposit_byte(self, bits: u8, shift: u32) -> Self;
}

macro_rules! impl_bit_int {
    ($($t:ty),*) => {$(
        impl BitInt for $t {
            const BITS: u32 = <$t>::BITS;
            const ZERO: Self = 0;
            #[inline(always)]
            fn extract_byte(self, shift: u32) -> u8 { (self >> shift) as u8 }
            #[inline(always)]
            fn deposit_byte(self, bits: u8, shift: u32) -> Self {
                self | ((bits as $t) << shift)
            }
        }
    )*};
}
impl_bit_int!(u8, u16, u32, u64, u128, usize);

macro_rules! impl_bit_codec {
    ($($ty:ty),*) => {
        $(
            impl BitEncode for $ty {
                #[inline]
                fn encode<W: std::io::Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
                    w.write_full_bytes(&self.to_le_bytes())
                }
            }
            impl BitDecode for $ty {
                #[inline]
                fn decode<R: std::io::Read>(r: &mut BitReader<R>) -> io::Result<Self> {
                    let mut buf = [0u8; std::mem::size_of::<$ty>()];
                    r.read_full_bytes(&mut buf)?;
                    Ok(<$ty>::from_le_bytes(buf))
                }
            }
        )*
    };
}

impl_bit_codec!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64);

pub trait BitVecHeader {
    fn write_len<W: Write>(len: usize, w: &mut BitWriter<W>) -> io::Result<()>;
    fn read_len<R: Read>(r: &mut BitReader<R>) -> io::Result<usize>;
}

impl<T: BitEncode + BitVecHeader> BitEncode for [T] {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        T::write_len(self.len(), w)?;
        for item in self {
            item.encode(w)?;
        }
        Ok(())
    }
}

impl<T: BitEncode + BitVecHeader> BitEncode for Vec<T> {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        self.as_slice().encode(w)
    }
}

impl<T: BitDecode + BitVecHeader> BitDecode for Vec<T> {
    fn decode<R: Read>(r: &mut BitReader<R>) -> io::Result<Self> {
        let len = T::read_len(r)?;
        (0..len).map(|_| T::decode(r)).collect()
    }
}

#[macro_export]
macro_rules! impl_bit_vec_header {
    ($header_ty:ty, $($ty:ty),*) => {
        $(
            impl $crate::storage::io::BitVecHeader for $ty {
                fn write_len<W: std::io::Write>(len: usize, w: &mut BitWriter<W>) -> std::io::Result<()> {
                    (len as $header_ty).encode(w)
                }
                fn read_len<R: std::io::Read>(r: &mut BitReader<R>) -> std::io::Result<usize> {
                    Ok(<$header_ty>::decode(r)? as usize)
                }
            }
        )*
    };
}
impl_bit_vec_header!(
    u32, u8, u16, u32, u64, u128, i8, i16, i32, i64, i128, f32, f64, String
);

impl<T: BitEncode> BitEncode for Option<T> {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        w.write(&self.is_some())?;
        if let Some(v) = self {
            v.encode(w)?;
        }
        Ok(())
    }
}

impl<T: BitDecode> BitDecode for Option<T> {
    fn decode<R: Read>(r: &mut BitReader<R>) -> io::Result<Self> {
        if r.read::<bool>()? {
            Ok(Some(T::decode(r)?))
        } else {
            Ok(None)
        }
    }
}

impl BitEncode for bool {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        w.write_bits(*self as u8, 1)
    }
}

impl BitDecode for bool {
    fn decode<R: Read>(r: &mut BitReader<R>) -> io::Result<Self> {
        r.read_bits::<u8>(1).map(|b| b != 0)
    }
}

impl BitEncode for String {
    fn encode<W: Write>(&self, w: &mut BitWriter<W>) -> io::Result<()> {
        (self.len() as u32).encode(w)?;
        w.write_full_bytes(self.as_bytes())
    }
}

impl BitDecode for String {
    fn decode<R: Read>(r: &mut BitReader<R>) -> io::Result<Self> {
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

        let mut r = BitReader::new(buf.as_slice());
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

        let mut r = BitReader::new(buf.as_slice());
        assert!(r.read::<bool>().unwrap());
        assert!(!r.read::<bool>().unwrap());
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

        let mut r = BitReader::new(buf.as_slice());
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

        let mut r = BitReader::new(buf.as_slice());
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
            w.write_bits(0b110u32, 3).unwrap();
            w.write(&true).unwrap();
            w.write(&0xFFu8).unwrap();
            w.write(&1024u16).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(buf.as_slice());
        assert_eq!(r.read_bits::<u8>(3).unwrap(), 0b110);
        assert!(r.read::<bool>().unwrap());
        assert_eq!(r.read::<u8>().unwrap(), 0xFF);
        assert_eq!(r.read::<u16>().unwrap(), 1024);
    }

    #[test]
    fn nibbles() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(0xAu32, 4).unwrap();
            w.write_bits(0x5u32, 4).unwrap();
            w.flush().unwrap();
        }
        assert_eq!(buf, [0xA5]);

        let mut r = BitReader::new(buf.as_slice());
        assert_eq!(r.read_bits::<u8>(4).unwrap(), 0xA);
        assert_eq!(r.read_bits::<u8>(4).unwrap(), 0x5);
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

        let mut r = BitReader::new(buf.as_slice());
        let mut out = vec![0u8; original.len()];
        r.read_full_bytes(&mut out).unwrap();
        assert_eq!(&out, original);
    }

    #[test]
    fn alignment() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            w.write_bits(0b111u8, 3).unwrap();
            w.align().unwrap();
            w.write(&0xABu8).unwrap();
            w.flush().unwrap();
        }
        assert_eq!(buf, [0b1110_0000, 0xAB]);
    }

    #[test]
    fn std_read_trait() {
        let data: &[u8] = &[0x01, 0x02, 0x03, 0x04];
        let mut r = BitReader::new(data);
        let mut buf = [0u8; 4];
        r.read_exact(&mut buf).unwrap();
        assert_eq!(buf, [0x01, 0x02, 0x03, 0x04]);
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
        let data: &[u8] = &[0xFF];
        let mut r = BitReader::new(data);
        r.read::<u8>().unwrap();
        assert!(r.read_bits::<u8>(1).is_err());
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

        let mut r = BitReader::new(buf.as_slice());
        assert_eq!(r.read::<f32>().unwrap(), std::f32::consts::PI);
        assert_eq!(r.read::<f64>().unwrap(), std::f64::consts::E);
    }

    #[test]
    fn round_trip_slice() {
        let mut buf = Vec::new();
        {
            let mut w = BitWriter::new(&mut buf);
            let data: &[u16] = &[10, 20, 30];
            w.write(data).unwrap();
            w.flush().unwrap();
        }

        let mut r = BitReader::new(buf.as_slice());
        assert_eq!(r.read::<Vec<u16>>().unwrap(), vec![10, 20, 30]);
    }
}
