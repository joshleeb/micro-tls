use crate::msgs::{Codec, CodecLength, Decoder, Encoder};
use managed::ManagedSlice;

pub enum Slice<'a, T: Codec<'a> + CodecLength<'a>> {
    Encodable { items: ManagedSlice<'a, T> },
    Decodable { len: usize, dec: Decoder<'a> },
}

impl<'a, T> Slice<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
    pub fn len(&self) -> usize {
        match self {
            Slice::Encodable { items } => items.len(),
            Slice::Decodable { len, .. } => *len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, T> Codec<'a> for Slice<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
    fn encode(&self, enc: &mut Encoder<'a>) {
        if !self.is_empty() {
            if let Slice::Encodable { items } = self {
                let len = items.len() * T::LENGTH;
                T::encode_len(len, enc);
                items.iter().for_each(|x| x.encode(enc));
            } else {
                panic!("cannot encode a decodable slice");
            }
        }
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        if !dec.is_complete() {
            let len = T::decode_len(dec)?;

            Some(Slice::Decodable {
                len,
                dec: dec.sub(len)?,
            })
        } else {
            Some(Slice::Decodable {
                len: 0,
                dec: dec.sub(0)?,
            })
        }
    }
}

impl<'a, T, Ms> From<Ms> for Slice<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
    Ms: Into<ManagedSlice<'a, T>>,
{
    fn from(items: Ms) -> Self {
        Slice::Encodable {
            items: items.into(),
        }
    }
}

impl<'a, T> Iterator for Slice<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Slice::Encodable { .. } => None,
            Slice::Decodable { dec, .. } => T::decode(dec),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn empty() {
            let items: Slice<'_, u8> = vec![].into();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert!(enc.bytes().is_empty());
        }

        #[test]
        fn single_byte_items() {
            let items: Slice<'_, u8> = vec![0, 1, 2, 3].into();
            let mut enc = Encoder::new(vec![]);
            items.encode(&mut enc);

            assert_eq!(enc.bytes(), [4, 0, 1, 2, 3]);
        }

        #[test]
        fn multiple_byte_items() {
            let items: Slice<'_, u16> = vec![0, 1, 2, 3].into();
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
            let slice: Slice<'_, u8> = Slice::decode(&mut dec).unwrap();

            assert!(slice.is_empty());
        }

        #[test]
        fn zero_length() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let slice: Slice<'_, u8> = Slice::decode(&mut dec).unwrap();

            assert!(slice.is_empty());
        }

        #[test]
        fn single_byte_items() {
            let bytes = [4, 0, 1, 2, 3];
            let mut dec = Decoder::new(&bytes);
            let slice: Slice<'_, u8> = Slice::decode(&mut dec).unwrap();

            assert_eq!(slice.collect::<Vec<u8>>(), vec![0, 1, 2, 3]);
        }

        #[test]
        fn multiple_byte_items() {
            let bytes = [0, 8, 0, 0, 0, 1, 0, 2, 0, 3];
            let mut dec = Decoder::new(&bytes);
            let slice: Slice<'_, u16> = Slice::decode(&mut dec).unwrap();

            assert_eq!(slice.collect::<Vec<u16>>(), vec![0, 1, 2, 3]);
        }
    }
}
