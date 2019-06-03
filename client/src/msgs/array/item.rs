use crate::msgs::{array::iter::ArrayIter, Codec, CodecSized, Decoder, Encoder};
use core::borrow::Borrow;

#[derive(Debug, Clone, PartialEq)]
pub enum Item<'a, T: Codec<'a>> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T: PartialEq + Codec<'a>> PartialEq<T> for Item<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other
    }
}

impl<'a, T: Codec<'a>> AsRef<T> for Item<'a, T> {
    fn as_ref(&self) -> &T {
        self.borrow()
    }
}

impl<'a, T: Codec<'a>> Borrow<T> for Item<'a, T> {
    fn borrow(&self) -> &T {
        match self {
            Item::Borrowed(ref_t) => ref_t,
            Item::Owned(t) => &t,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn empty() {
        let items: Items<'_, u8> = Items::empty();

        assert_eq!(items, Items::Typed(&[]));
    }

    #[test]
    fn typed_iter() {
        let items: Items<'_, u8> = Items::Typed(&[7, 8, 9]);

        assert_eq!(items.iter().collect::<Vec<Item<'_, u8>>>(), vec![7, 8, 9]);
    }

    #[test]
    fn single_bytes_iter() {
        let items: Items<'_, u8> = Items::Bytes(&[7, 8, 9]);

        assert_eq!(items.iter().collect::<Vec<Item<'_, u8>>>(), vec![7, 8, 9]);
    }

    #[test]
    fn multi_bytes_iter() {
        let items: Items<'_, u16> = Items::Bytes(&[0, 7, 0, 8, 0, 9]);

        assert_eq!(items.iter().collect::<Vec<Item<'_, u16>>>(), vec![7, 8, 9]);
    }

    mod encode {
        use super::*;

        #[test]
        fn empty() {
            let items: Items<'_, u8> = Items::empty();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert!(enc.bytes().is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn single_byte_items() {
            let items: Items<'_, u8> = Items::from([7, 8, 9].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [7, 8, 9]);
            assert_eq!(items.data_size(), 3);
        }

        #[test]
        fn multi_byte_items() {
            let items: Items<'_, u16> = Items::from([7, 8, 9].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0, 7, 0, 8, 0, 9]);
            assert_eq!(items.data_size(), 6);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty() {
            let bytes = [];
            let mut dec = Decoder::new(&bytes);
            let items: Items<'_, u8> = Items::decode(0, &mut dec).unwrap();

            assert_eq!(items, Items::Bytes(&bytes));
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn single_byte_items() {
            let bytes = [7, 8, 9];
            let mut dec = Decoder::new(&bytes);
            let items: Items<'_, u8> = Items::decode(3, &mut dec).unwrap();

            assert_eq!(items, Items::Bytes(&bytes));
            assert_eq!(items.data_size(), 3);
        }

        #[test]
        fn multi_byte_items() {
            let bytes = [0, 7, 0, 8, 0, 9];
            let mut dec = Decoder::new(&bytes);
            let items: Items<'_, u16> = Items::decode(6, &mut dec).unwrap();

            assert_eq!(items, Items::Bytes(&bytes));
            assert_eq!(items.data_size(), 6);
        }
    }
}
