use crate::primitive::u24;
use core::{u16, u32, u8};
use decoder::Decoder;
use encoder::Encoder;

pub mod decoder;
pub mod encoder;

/// Data that can be encoded by an [`Encoder`] and decoded by a [`Decoder`].
pub trait Codec<'a>: Sized {
    fn encode(&self, _enc: &mut Encoder<'a>);

    fn decode(_dec: &mut Decoder<'a>) -> Option<Self>;
}

/// Data that can be encoded and decoded as part of an [`Array`](crate::array::Array).
///
/// TODO: CodecSized tests
pub trait CodecSized<'a>: Codec<'a> {
    const HEADER_SIZE: HeaderSize;

    // How many bytes when this is encoded?
    fn data_size(&self) -> usize;

    fn encode_len(&self, enc: &mut Encoder<'a>) {
        Self::HEADER_SIZE.encode_len(self.data_size(), enc)
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        Self::HEADER_SIZE.decode_len(dec)
    }
}

/// Size of the header of an encoded [`Array`](crate::array::Array) of items that implement
/// [`CodcSized`].
pub enum HeaderSize {
    /// No header.
    Zero,
    /// 1 byte header.
    U8,
    /// 2 byte header.
    U16,
    /// 3 byte header.
    U24,
    /// 4 byte header.
    U32,
}

impl HeaderSize {
    pub fn size(&self) -> usize {
        match self {
            HeaderSize::Zero => 0,
            HeaderSize::U8 => 1,
            HeaderSize::U16 => 2,
            HeaderSize::U24 => 3,
            HeaderSize::U32 => 4,
        }
    }

    fn encode_len<'a>(&self, len: usize, enc: &mut Encoder<'a>) {
        match self {
            HeaderSize::Zero => {}
            HeaderSize::U8 => HeaderSize::as_u8(len).encode(enc),
            HeaderSize::U16 => HeaderSize::as_u16(len).encode(enc),
            HeaderSize::U24 => HeaderSize::as_u24(len).encode(enc),
            HeaderSize::U32 => HeaderSize::as_u32(len).encode(enc),
        }
    }

    fn decode_len<'a>(&self, dec: &mut Decoder<'a>) -> Option<usize> {
        match self {
            HeaderSize::Zero => None,
            HeaderSize::U8 => u8::decode(dec).map(usize::from),
            HeaderSize::U16 => u16::decode(dec).map(usize::from),
            HeaderSize::U24 => u24::decode(dec).map(u24::as_u32).map(|x| x as usize),
            HeaderSize::U32 => u32::decode(dec).map(|x| x as usize),
        }
    }

    fn as_u8(data: usize) -> u8 {
        debug_assert!(data <= usize::from(u8::MAX));
        data as u8
    }

    fn as_u16(data: usize) -> u16 {
        debug_assert!(data <= usize::from(u16::MAX));
        data as u16
    }

    fn as_u24(data: usize) -> u24 {
        debug_assert!(data <= usize::pow(2, 24) - 1);
        u24::from(data as u32)
    }

    // TODO: Fix as_u32 which may panic on 16 or 32 bit devices?
    fn as_u32(data: usize) -> u32 {
        debug_assert!(data <= u32::MAX as usize);
        data as u32
    }
}
