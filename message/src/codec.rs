use core::{u16, u8};
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
    // TODO: Replace usize with enum to remove the unimplemented catch all in the match statements.
    // How many bytes should data_size() be put into?
    const HEADER_SIZE: usize;

    // How many bytes when this is encoded?
    fn data_size(&self) -> usize;

    fn encode_len(&self, enc: &mut Encoder<'a>) {
        match Self::HEADER_SIZE {
            0 => {}
            1 => self.encode_u8(enc),
            2 => self.encode_u16(enc),
            _ => unimplemented!(),
        }
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        match Self::HEADER_SIZE {
            0 => None,
            1 => u8::decode(dec).map(usize::from),
            2 => u16::decode(dec).map(usize::from),
            _ => unimplemented!(),
        }
    }

    fn encode_u8(&self, enc: &mut Encoder<'a>) {
        let n_bytes = self.data_size();
        debug_assert!(n_bytes <= usize::from(u8::MAX));
        (n_bytes as u8).encode(enc);
    }

    fn encode_u16(&self, enc: &mut Encoder<'a>) {
        let n_bytes = self.data_size();
        debug_assert!(n_bytes <= usize::from(u16::MAX));
        (n_bytes as u16).encode(enc);
    }
}
