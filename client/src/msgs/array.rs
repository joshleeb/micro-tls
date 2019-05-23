use crate::msgs::{Codec, CodecLength, Decoder, Encoder};
use core::{marker::PhantomData, slice};

pub struct Array<'a, T: Codec<'a> + CodecLength<'a>> {
    len: usize,
    items: Items<'a, T>,
}

impl<'a, T> Array<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
    pub fn empty() -> Self {
        Self {
            len: 0,
            items: Items::Typed(&[]),
        }
    }

    pub fn iter(&self) -> ArrayIter<'a, T> {
        match self.items {
            Items::Typed(t) => ArrayIter::Typed(TypedArrayIter { it: t.iter() }),
            Items::Bytes(b) => ArrayIter::Bytes(BytesArrayIter {
                dec: Decoder::new(b),
                phantom: PhantomData,
            }),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> Codec<'a> for Array<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
    fn encode(&self, enc: &mut Encoder<'a>) {
        if self.is_empty() {
            return;
        }

        let n_bytes = self.len() * T::LENGTH;
        T::encode_len(n_bytes, enc);
        self.items.encode(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        if dec.is_complete() {
            return Some(Array::empty());
        }

        let len = T::decode_len(dec)?;
        Items::decode(len, dec).map(|items| Self { len, items })
    }
}

impl<'a, T> From<&'a [T]> for Array<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
    fn from(items: &'a [T]) -> Self {
        Self {
            len: items.len(),
            items: Items::Typed(items),
        }
    }
}

pub enum ArrayIter<'a, T: Codec<'a>> {
    Typed(TypedArrayIter<'a, T>),
    Bytes(BytesArrayIter<'a, T>),
}

impl<'a, T: Copy + Codec<'a>> Iterator for ArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayIter::Typed(t) => t.next(),
            ArrayIter::Bytes(b) => b.next(),
        }
    }
}

struct TypedArrayIter<'a, T: Codec<'a>> {
    it: slice::Iter<'a, T>,
}

impl<'a, T: Copy + Codec<'a>> Iterator for TypedArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().copied()
    }
}

struct BytesArrayIter<'a, T: Codec<'a>> {
    dec: Decoder<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T: Codec<'a>> Iterator for BytesArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::decode(&mut self.dec)
    }
}

enum Items<'a, T: Codec<'a>> {
    Typed(&'a [T]),
    Bytes(&'a [u8]),
}

impl<'a, T: Codec<'a>> Items<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        match self {
            Items::Typed(t) => t.iter().for_each(|item| item.encode(enc)),
            Items::Bytes(b) => enc.append(b),
        }
    }

    fn decode(len: usize, dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(len).map(Items::Bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_len() {
        let items: Array<'_, u8> = Array::from([].as_ref());

        assert_eq!(items.len(), 0);
        assert!(items.is_empty());
    }

    #[test]
    fn non_empty_len() {
        let items: Array<'_, u8> = Array::from([99].as_ref());

        assert_eq!(items.len(), 1);
        assert!(!items.is_empty());
    }

    mod encode {
        use super::*;

        #[test]
        fn empty() {
            let items: Array<'_, u8> = Array::from([].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert!(enc.bytes().is_empty());
        }

        #[test]
        fn single_byte_items() {
            let items: Array<'_, u8> = Array::from([0, 1, 2, 3].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [4, 0, 1, 2, 3]);
        }

        #[test]
        fn multiple_byte_items() {
            let items: Array<'_, u16> = Array::from([0, 1, 2, 3].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0, 8, 0, 0, 0, 1, 0, 2, 0, 3]);
        }
    }

    mod decode {
        use super::*;
        use std::vec::Vec;

        #[test]
        fn empty() {
            let bytes = [];
            let mut dec = Decoder::new(&bytes);
            let slice: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(slice.is_empty());
        }

        #[test]
        fn zero_length() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let slice: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(slice.is_empty());
        }

        #[test]
        fn single_byte_items() {
            let bytes = [4, 0, 1, 2, 3];
            let mut dec = Decoder::new(&bytes);
            let slice: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert_eq!(slice.iter().collect::<Vec<u8>>(), vec![0, 1, 2, 3]);
        }

        #[test]
        fn multiple_byte_items() {
            let bytes = [0, 8, 0, 0, 0, 1, 0, 2, 0, 3];
            let mut dec = Decoder::new(&bytes);
            let slice: Array<'_, u16> = Array::decode(&mut dec).unwrap();

            assert_eq!(slice.iter().collect::<Vec<u16>>(), vec![0, 1, 2, 3]);
        }
    }
}
