use crate::msgs::{array::iter::ArrayIter, Codec, Decoder, Encoder};

#[derive(Debug, Clone)]
pub enum Items<'a, T: Codec<'a>> {
    Typed(&'a [T]),
    Bytes(&'a [u8]),
}

impl<'a, T: Codec<'a>> Items<'a, T> {
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

impl<'a, T: Codec<'a>> From<&'a [T]> for Items<'a, T> {
    fn from(inner: &'a [T]) -> Self {
        Items::Typed(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        // TODO: array::item encode tests
    }

    mod decode {
        use super::*;

        // TODO: array::item decode tests
    }
}
