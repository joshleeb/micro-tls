pub use decoder::Decoder;
pub use encoder::Encoder;

pub(crate) use header::HeaderSize;

#[macro_use]
pub(crate) mod array;

pub(crate) mod num;

mod decoder;
mod encoder;
mod header;

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
