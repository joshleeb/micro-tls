use crate::codec::{Codec, CodecSized, Decoder, Encoder, HeaderSize};

#[derive(Debug, Default, PartialEq)]
pub struct Random([u8; 32]);

impl<'a> Codec<'a> for Random {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.append(&self.0);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let bytes = dec.take(32)?;
        let mut opaque = [0; 32];
        opaque.clone_from_slice(bytes);

        Some(Random(opaque))
    }
}

impl<'a> CodecSized<'a> for Random {
    const HEADER_SIZE: HeaderSize = HeaderSize::Zero;

    fn data_size(&self) -> usize {
        self.0.len()
    }
}

impl PartialEq<[u8; 32]> for Random {
    fn eq(&self, other: &[u8; 32]) -> bool {
        self.0 == *other
    }
}

impl From<[u8; 32]> for Random {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_size() {
        let random = Random::default();
        assert_eq!(random.data_size(), 32);
    }

    mod encode {
        use super::*;

        #[test]
        fn multiple_bytes() {
            let random = Random::default();
            let mut enc = Encoder::new(vec![]);
            random.encode(&mut enc);

            assert_eq!(enc.bytes(), [0; 32]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn not_enough_bytes() {
            let bytes = [0; 31];
            let mut dec = Decoder::new(&bytes);
            let random = Random::decode(&mut dec);

            assert!(random.is_none());
        }

        #[test]
        fn multiple_bytes() {
            let bytes = [0; 32];
            let mut dec = Decoder::new(&bytes);
            let random = Random::decode(&mut dec).unwrap();

            assert_eq!(random, Random::default());
        }
    }
}
