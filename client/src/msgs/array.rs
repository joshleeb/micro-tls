use crate::msgs::{Codec, CodecSized, Decoder, Encoder};
use item::Items;
use iter::ArrayIter;

#[macro_use]
pub mod macros;

pub mod iter;

mod item;

#[derive(Debug, Clone)]
pub struct Array<'a, T: Codec<'a> + CodecSized<'a>> {
    items: Items<'a, T>,
    len: usize,
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Array<'a, T> {
    pub fn empty() -> Self {
        Self {
            items: Items::empty(),
            len: 0,
        }
    }

    pub fn iter(&self) -> ArrayIter<'a, T> {
        self.items.iter()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Codec<'a> for Array<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.encode_len(enc);
        self.items.encode(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        if dec.is_complete() {
            return Some(Array::empty());
        }

        let len = Self::decode_len(dec)?;
        Items::decode(len, dec).map(|items| Self { len, items })
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> CodecSized<'a> for Array<'a, T> {
    const HEADER_SIZE: usize = T::HEADER_SIZE;

    fn data_size(&self) -> usize {
        self.items.data_size()
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Default for Array<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> From<&'a [T]> for Array<'a, T> {
    fn from(items: &'a [T]) -> Self {
        Self {
            items: items.into(),
            len: items.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec::Vec;

    #[test]
    fn empty_len() {
        let items: Array<'_, u8> = arr![];

        assert_eq!(items.len(), 0);
        assert!(items.is_empty());
    }

    #[test]
    fn single_item() {
        let items: Array<'_, u8> = arr![99];

        assert_eq!(items.len(), 1);
        assert_eq!(items.iter().collect::<Vec<u8>>(), vec![99]);
    }

    #[test]
    fn multi_items() {
        let items: Array<'_, u8> = arr![98, 99];

        assert_eq!(items.len(), 2);
        assert_eq!(items.iter().collect::<Vec<u8>>(), vec![98, 99]);
    }

    mod encode {
        use super::*;

        #[test]
        fn empty_single_byte_size() {
            let items: Array<'_, u8> = Array::empty();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0]);
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn empty_multi_bytes_size() {
            let items: Array<'_, u16> = Array::empty();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0, 0]);
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn single_byte_items() {
            let items: Array<'_, u8> = arr![96, 97, 98, 99];
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [4, 96, 97, 98, 99]);
            assert_eq!(items.data_size(), 4);
        }

        #[test]
        fn multiple_byte_items() {
            let items: Array<'_, u16> = arr![96, 97, 98, 99];
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [0, 8, 0, 96, 0, 97, 0, 98, 0, 99]);
            assert_eq!(items.data_size(), 8);
        }
    }

    mod decode {
        use super::*;
        use std::vec::Vec;

        #[test]
        fn empty_single_byte_size() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn empty_multi_byte_size() {
            let bytes = [0, 0];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u16> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn empty_multi_byte_size_invalid() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let items: Option<Array<'_, u16>> = Array::decode(&mut dec);

            assert!(items.is_none());
        }

        #[test]
        fn zero_length() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn single_byte_items() {
            let bytes = [4, 96, 97, 98, 99];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert_eq!(items.iter().collect::<Vec<u8>>(), vec![96, 97, 98, 99]);
            assert_eq!(items.data_size(), 4);
        }

        #[test]
        fn multiple_byte_items() {
            let bytes = [0, 8, 0, 96, 0, 97, 0, 98, 0, 99];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u16> = Array::decode(&mut dec).unwrap();

            assert_eq!(items.iter().collect::<Vec<u16>>(), vec![96, 97, 98, 99]);
            assert_eq!(items.data_size(), 8);
        }
    }
}
