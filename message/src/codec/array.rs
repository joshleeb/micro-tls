use crate::codec::{decoder::Decoder, encoder::Encoder, Codec, CodecSized, HeaderSize};
use iter::ArrayIter;

#[macro_use]
mod macros;

pub mod item;
pub mod iter;

#[derive(Debug, Clone)]
pub enum Array<'a, T: CodecSized<'a>> {
    Typed(&'a [T]),
    Bytes(&'a [u8]),
}

impl<'a, T: CodecSized<'a>> Array<'a, T> {
    pub fn empty() -> Self {
        arr![]
    }

    pub fn encode_items(&self, enc: &mut Encoder<'a>) {
        match self {
            Array::Typed(t) => t.iter().for_each(|item| item.encode(enc)),
            Array::Bytes(b) => enc.append(b),
        }
    }

    pub fn decode_items(len: usize, dec: &mut Decoder<'a>) -> Option<Self> {
        let bytes = dec.take(len)?;
        Some(Array::Bytes(bytes))
    }

    pub fn iter(&self) -> ArrayIter<'a, T> {
        match self {
            Array::Typed(t) => ArrayIter::from(t.iter()),
            Array::Bytes(b) => ArrayIter::from(Decoder::new(b)),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            Array::Typed(t) => t.len(),
            Array::Bytes(b) => b.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T: CodecSized<'a>> Codec<'a> for Array<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.encode_len(enc);
        match self {
            Array::Typed(t) => t.iter().for_each(|item| item.encode(enc)),
            Array::Bytes(b) => enc.append(b),
        }
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        if dec.is_complete() {
            return Some(Array::empty());
        }
        T::decode_len(dec).and_then(|len| Self::decode_items(len, dec))
    }
}

impl<'a, T: CodecSized<'a>> CodecSized<'a> for Array<'a, T> {
    const HEADER_SIZE: HeaderSize = T::HEADER_SIZE;

    fn data_size(&self) -> usize {
        match self {
            Array::Typed(t) => t.iter().map(CodecSized::data_size).sum::<usize>(),
            Array::Bytes(b) => b.len(),
        }
    }
}

impl<'a, T: CodecSized<'a>> Default for Array<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T: CodecSized<'a>> From<&'a [T]> for Array<'a, T> {
    fn from(inner: &'a [T]) -> Self {
        Array::Typed(inner)
    }
}

impl<'a, T: PartialEq + CodecSized<'a>> PartialEq for Array<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Array::Typed(t1), Array::Typed(t2)) => t1 == t2,
            (Array::Bytes(b1), Array::Bytes(b2)) => b1 == b2,
            _ => self.iter().zip(other.iter()).all(|(a, b)| a == b),
        }
    }
}
impl<'a, T: PartialEq + CodecSized<'a>> Eq for Array<'a, T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use item::Item;
    use std::vec::Vec;

    #[test]
    fn empty() {
        let items: Array<'_, u8> = Array::empty();

        assert_eq!(items, Array::Typed(&[]));
    }

    #[test]
    fn typed_iter() {
        let items: Array<'_, u8> = Array::Typed(&[7, 8, 9]);

        assert_eq!(items.iter().collect::<Vec<Item<'_, u8>>>(), vec![7, 8, 9]);
    }

    #[test]
    fn single_bytes_iter() {
        let items: Array<'_, u8> = Array::Bytes(&[7, 8, 9]);

        assert_eq!(items.iter().collect::<Vec<Item<'_, u8>>>(), vec![7, 8, 9]);
    }

    #[test]
    fn multi_bytes_iter() {
        let items: Array<'_, u16> = Array::Bytes(&[0, 7, 0, 8, 0, 9]);

        assert_eq!(items.iter().collect::<Vec<Item<'_, u16>>>(), vec![7, 8, 9]);
    }

    #[test]
    fn array_empty_eq() {
        assert_eq!(Array::<u32>::default(), Array::<u32>::default());
    }

    #[test]
    fn array_typed_eq() {
        assert_eq!(
            Array::<u32>::Typed(&[1, 2, 3]),
            Array::<u32>::Typed(&[1, 2, 3])
        );
    }

    #[test]
    fn array_typed_ne() {
        assert_ne!(
            Array::<u32>::Typed(&[1, 2, 3]),
            Array::<u32>::Typed(&[4, 5, 6])
        );
    }

    #[test]
    fn array_bytes_eq() {
        assert_eq!(
            Array::<u8>::Bytes(&[1, 2, 3]),
            Array::<u8>::Bytes(&[1, 2, 3])
        );
    }

    #[test]
    fn array_bytes_ne() {
        assert_ne!(
            Array::<u8>::Bytes(&[1, 2, 3]),
            Array::<u8>::Bytes(&[4, 5, 6])
        );
    }

    #[test]
    fn array_mixed_eq() {
        assert_eq!(
            Array::<u16>::Typed(&[1, 2, 3]),
            Array::<u16>::Bytes(&[0, 1, 0, 2, 0, 3])
        );
    }

    #[test]
    fn array_mixed_ne() {
        assert_ne!(
            Array::<u16>::Typed(&[1, 2, 3]),
            Array::<u16>::Bytes(&[0, 4, 0, 5, 0, 6])
        );
    }

    mod encode {
        use super::*;

        #[test]
        fn empty_single_bytes_size() {
            let items: Array<'_, u8> = Array::empty();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0]);
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn empty_multiple_bytes_size() {
            let items: Array<'_, u16> = Array::empty();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0, 0]);
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn single_byte_items() {
            let items: Array<'_, u8> = Array::from([7, 8, 9].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [3, 7, 8, 9]);
            assert_eq!(items.data_size(), 3);
        }

        #[test]
        fn multi_byte_items() {
            let items: Array<'_, u16> = Array::from([7, 8, 9].as_ref());
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0, 6, 0, 7, 0, 8, 0, 9]);
            assert_eq!(items.data_size(), 6);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty() {
            let bytes = [];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn zero_length_single_bytes_size() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn zero_length_multiple_byte_size() {
            let bytes = [0, 0];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u16> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn zero_length_multiple_byte_size_invalid() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let items: Option<Array<'_, u16>> = Array::decode(&mut dec);

            assert!(items.is_none());
        }

        #[test]
        fn single_byte_items() {
            let bytes = [3, 7, 8, 9];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert_eq!(items, Array::Bytes(&[7, 8, 9]));
            assert_eq!(items.data_size(), 3);
        }

        #[test]
        fn multi_byte_items() {
            let bytes = [0, 6, 0, 7, 0, 8, 0, 9];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u16> = Array::decode(&mut dec).unwrap();

            assert_eq!(items, Array::Bytes(&[0, 7, 0, 8, 0, 9]));
            assert_eq!(items.data_size(), 6);
        }
    }
}
