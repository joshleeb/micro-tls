use crate::msgs::{array::iter::ArrayIter, Codec, CodecSized, Decoder, Encoder};

#[derive(Debug, Clone)]
pub enum Items<'a, T: Codec<'a> + CodecSized<'a>> {
    Typed(&'a [T]),
    Bytes(&'a [u8]),
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Items<'a, T> {
    pub fn empty() -> Self {
        Items::Typed(&[])
    }

    pub fn iter(&self) -> ArrayIter<'a, T> {
        match self {
            Items::Typed(t) => ArrayIter::from(t.iter()),
            Items::Bytes(b) => ArrayIter::from(Decoder::new(b)),
        }
    }

    pub fn encode(&self, enc: &mut Encoder<'a>) {
        match self {
            Items::Typed(t) => t.iter().for_each(|item| item.encode(enc)),
            Items::Bytes(b) => enc.append(b),
        }
    }

    pub fn decode(len: usize, dec: &mut Decoder<'a>) -> Option<Self> {
        let bytes = dec.take(len)?;
        Some(Items::Bytes(bytes))
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> CodecSized<'a> for Items<'a, T> {
    const HEADER_SIZE: usize = T::HEADER_SIZE;

    // TODO: CodeLength::LENGTH for u16 should look into using mem::size_of
    fn data_size(&self) -> usize {
        match self {
            Items::Typed(t) => t.iter().map(CodecSized::data_size).sum::<usize>(),
            Items::Bytes(b) => b.len(),
        }
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> From<&'a [T]> for Items<'a, T> {
    fn from(inner: &'a [T]) -> Self {
        Items::Typed(inner)
    }
}
