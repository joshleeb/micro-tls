use crate::msgs::{Codec, CodecSized, Decoder, Encoder};
use item::Items;
use iter::ArrayIter;

#[macro_use]
pub mod macros;

pub mod item;
pub mod iter;

#[derive(Debug, Clone)]
pub struct Array<'a, T: Codec<'a> + CodecSized<'a>> {
    items: Items<'a, T>,
    len: usize,
}

impl<'a, T> PartialEq for Array<'a, T>
where
    T: PartialEq + Codec<'a> + CodecSized<'a>,
{
    fn eq(&self, other: &Self) -> bool {
        self.items == other.items
    }
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

    pub(crate) fn encode_items(&self, enc: &mut Encoder<'a>) {
        self.items.encode(enc);
    }

    pub(crate) fn decode_items(len: usize, dec: &mut Decoder<'a>) -> Option<Self> {
        Items::decode(len, dec).map(|items| Self { len, items })
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Codec<'a> for Array<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.encode_len(enc);
        self.encode_items(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        if dec.is_complete() {
            return Some(Array::empty());
        }

        T::decode_len(dec).and_then(|len| Self::decode_items(len, dec))
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
    use crate::msgs::array::item::Item;
    use std::vec::Vec;

    #[test]
    fn empty_len() {
        let items: Array<'_, u8> = arr![];

        assert!(items.is_empty());
    }

    #[test]
    fn single_item() {
        let items: Array<'_, u8> = arr![99];

        assert_eq!(items.len(), 1);
        assert_eq!(items.iter().collect::<Vec<Item<'_, u8>>>(), vec![99]);
    }

    #[test]
    fn multiple_items() {
        let items: Array<'_, u8> = arr![98, 99];

        assert_eq!(items.len(), 2);
        assert_eq!(items.iter().collect::<Vec<Item<'_, u8>>>(), vec![98, 99]);
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
        fn empty_multiple_bytes_size() {
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
        fn empty() {
            let bytes = [];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert!(items.is_empty());
            assert_eq!(items.data_size(), 0);
        }

        #[test]
        fn zero_length_single_byte_size() {
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
            let bytes = [4, 96, 97, 98, 99];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u8> = Array::decode(&mut dec).unwrap();

            assert_eq!(items, arr![96, 97, 98, 99]);
            assert_eq!(items.data_size(), 4);
        }

        #[test]
        fn multiple_byte_items() {
            let bytes = [0, 8, 0, 96, 0, 97, 0, 98, 0, 99];
            let mut dec = Decoder::new(&bytes);
            let items: Array<'_, u16> = Array::decode(&mut dec).unwrap();

            assert_eq!(items, arr![96, 97, 98, 99]);
            assert_eq!(items.data_size(), 8);
        }
    }
}
