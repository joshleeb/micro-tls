use crate::msgs::{Codec, Decoder, Encoder};

#[derive(Debug, Default, PartialEq)]
pub struct Random([u8; 32]);

impl<'a> Codec<'a> for Random {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.append(&self.0);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let bytes = dec.take(32)?.clone();
        let mut opaque = [0; 32];
        opaque.clone_from_slice(bytes);

        Some(Random(opaque))
    }
}

impl Random {
    pub fn empty() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn multiple_bytes() {
            let random = Random::empty();
            let mut enc = Encoder::new(vec![]);
            random.encode(&mut enc);

            assert_eq!(enc.bytes(), [0; 32]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn not_enough_bytes() {
            let mut dec = Decoder::new(vec![0; 31]);
            let random = Random::decode(&mut dec);

            assert!(random.is_none());
        }

        #[test]
        fn multiple_bytes() {
            let mut dec = Decoder::new(vec![0; 32]);
            let random = Random::decode(&mut dec).unwrap();

            assert_eq!(random, Random::empty());
        }
    }
}
