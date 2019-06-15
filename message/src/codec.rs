use core::{mem, u16, u8};
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
}

impl HeaderSize {
    pub fn size(&self) -> usize {
        match self {
            HeaderSize::U8 => mem::size_of::<u8>(),
            HeaderSize::U16 => mem::size_of::<u16>(),
            HeaderSize::Zero => 0,
        }
    }

    fn encode_len<'a>(&self, len: usize, enc: &mut Encoder<'a>) {
        match self {
            HeaderSize::U8 => HeaderSize::as_u8(len).encode(enc),
            HeaderSize::U16 => HeaderSize::as_u16(len).encode(enc),
            HeaderSize::Zero => {}
        }
    }

    fn decode_len<'a>(&self, dec: &mut Decoder<'a>) -> Option<usize> {
        match self {
            HeaderSize::U8 => u8::decode(dec).map(usize::from),
            HeaderSize::U16 => u16::decode(dec).map(usize::from),
            HeaderSize::Zero => None,
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
}
