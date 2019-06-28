use crate::{
    codec::{decoder::Decoder, encoder::Encoder, header::HeaderSize, Codec, CodecSized},
    error::Result as TlsResult,
};
use core::{u16, u32, u8};

impl<'a> Codec<'a> for u8 {
    fn encode(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        enc.push(*self)
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(Self::data_size(&0)).map(|b| b[0])
    }
}

impl<'a> CodecSized<'a> for u8 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U8;

    fn data_size(&self) -> usize {
        1
    }
}

impl<'a> Codec<'a> for u16 {
    fn encode(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        enc.append([(*self >> 8) as u8, *self as u8])
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(Self::data_size(&0))
            .map(|b| (u16::from(b[0]) << 8) | u16::from(b[1]))
    }
}

impl<'a> CodecSized<'a> for u16 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U16;

    fn data_size(&self) -> usize {
        2
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub struct u24(u32);

impl u24 {
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl<'a> Codec<'a> for u24 {
    fn encode(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        enc.append([(self.0 >> 16) as u8, (self.0 >> 8) as u8, self.0 as u8])
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(Self::data_size(&u24::from(0)))
            .map(|b| Self((u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2])))
    }
}

impl<'a> CodecSized<'a> for u24 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U24;

    fn data_size(&self) -> usize {
        3
    }
}

impl From<u32> for u24 {
    fn from(data: u32) -> Self {
        Self(data)
    }
}

impl<'a> Codec<'a> for u32 {
    fn encode(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        enc.append([
            (*self >> 24) as u8,
            (*self >> 16) as u8,
            (*self >> 8) as u8,
            *self as u8,
        ])
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(Self::data_size(&0)).map(|b| {
            (u32::from(b[0]) << 24)
                | (u32::from(b[1]) << 16)
                | (u32::from(b[2]) << 8)
                | u32::from(b[3])
        })
    }
}

impl<'a> CodecSized<'a> for u32 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U32;

    fn data_size(&self) -> usize {
        4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn u8_bytes() {
            let mut enc = Encoder::new(vec![]);
            1u8.encode(&mut enc).unwrap();

            assert_eq!(enc.bytes(), [1]);
        }

        #[test]
        fn u16_bytes() {
            let mut enc = Encoder::new(vec![]);
            1u16.encode(&mut enc).unwrap();

            assert_eq!(enc.bytes(), [0, 1]);
        }

        #[test]
        fn u24_bytes() {
            let mut enc = Encoder::new(vec![]);
            u24::from(1).encode(&mut enc).unwrap();

            assert_eq!(enc.bytes(), [0, 0, 1]);
        }

        #[test]
        fn u32_bytes() {
            let mut enc = Encoder::new(vec![]);
            1u32.encode(&mut enc).unwrap();

            assert_eq!(enc.bytes(), [0, 0, 0, 1]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn u8_bytes() {
            let mut dec = Decoder::new(&[1]);
            let n = u8::decode(&mut dec).unwrap();

            assert_eq!(n, 1);
        }

        #[test]
        fn u16_bytes() {
            let mut dec = Decoder::new(&[0, 1]);
            let n = u16::decode(&mut dec).unwrap();

            assert_eq!(n, 1);
        }

        #[test]
        fn u24_bytes() {
            let mut dec = Decoder::new(&[0, 0, 1]);
            let n = u24::decode(&mut dec).unwrap();

            assert_eq!(n, u24::from(1));
        }

        #[test]
        fn u32_bytes() {
            let mut dec = Decoder::new(&[0, 0, 0, 1]);
            let n = u32::decode(&mut dec).unwrap();

            assert_eq!(n, 1);
        }
    }
}
