use crate::codec::{decoder::Decoder, encoder::Encoder, Codec, CodecSized, HeaderSize};

#[derive(Debug, Default, PartialEq)]
pub struct SessionId {
    data: [u8; 32],
    len: usize,
}

impl SessionId {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<'a> Codec<'a> for SessionId {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.push(&(self.len as u8));
        enc.append(&self.data[..self.len]);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let len = u8::decode(dec)? as usize;
        if len > 32 {
            return None;
        }

        let bytes = dec.take(len)?;
        Some(bytes.into())
    }
}

impl<'a> CodecSized<'a> for SessionId {
    const HEADER_SIZE: HeaderSize = HeaderSize::Zero;

    fn data_size(&self) -> usize {
        assert!(self.len <= 32);
        u8::data_size(&(self.len as u8)) + self.len
    }
}

impl<T: AsRef<[u8]>> From<T> for SessionId {
    fn from(bytes: T) -> Self {
        let len = bytes.as_ref().len();
        let mut data = [0; 32];
        data[..len].copy_from_slice(&bytes.as_ref()[..len]);
        Self { data, len }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_data_size() {
        let session_id = SessionId::empty();
        assert_eq!(session_id.data_size(), 1);
    }

    #[test]
    fn data_size() {
        let session_id = SessionId::from([97, 98, 99]);
        assert_eq!(session_id.data_size(), 4);
    }

    mod encode {
        use super::*;

        #[test]
        fn empty() {
            let session_id = SessionId::empty();
            let mut enc = Encoder::new(vec![]);
            session_id.encode(&mut enc);

            assert_eq!(enc.bytes(), [0]);
        }

        #[test]
        fn single_bytes() {
            let mut data = [0; 32];
            data[0] = 99;

            let session_id = SessionId { data, len: 1 };
            let mut enc = Encoder::new(vec![]);
            session_id.encode(&mut enc);

            assert_eq!(enc.bytes(), [1, 99]);
        }

        #[test]
        fn multiple_bytes() {
            let mut data = [0; 32];
            data[0] = 99;
            data[1] = 98;
            data[2] = 97;

            let session_id = SessionId { data, len: 3 };
            let mut enc = Encoder::new(vec![]);
            session_id.encode(&mut enc);

            assert_eq!(enc.bytes(), [3, 99, 98, 97]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty() {
            let bytes = [];
            let mut dec = Decoder::new(&bytes);
            let session_id = SessionId::decode(&mut dec);

            assert!(session_id.is_none());
        }

        #[test]
        fn zero_length() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let session_id = SessionId::decode(&mut dec).unwrap();

            assert_eq!(session_id, SessionId::empty());
        }

        #[test]
        fn invalid_length() {
            let bytes = [33];
            let mut dec = Decoder::new(&bytes);
            let session_id = SessionId::decode(&mut dec);

            assert!(session_id.is_none());
        }

        #[test]
        fn not_enough_bytes() {
            let bytes = [2, 99];
            let mut dec = Decoder::new(&bytes);
            let session_id = SessionId::decode(&mut dec);

            assert!(session_id.is_none());
        }

        #[test]
        fn single_byte() {
            let bytes = [1, 99];
            let mut dec = Decoder::new(&bytes);
            let session_id = SessionId::decode(&mut dec).unwrap();

            let mut data = [0; 32];
            data[0] = 99;

            assert_eq!(session_id, SessionId { data, len: 1 });
        }

        #[test]
        fn multiple_bytes() {
            let bytes = [3, 99, 98, 97];
            let mut dec = Decoder::new(&bytes);
            let session_id = SessionId::decode(&mut dec).unwrap();

            let mut data = [0; 32];
            data[0] = 99;
            data[1] = 98;
            data[2] = 97;

            assert_eq!(session_id, SessionId { data, len: 3 });
        }
    }
}
