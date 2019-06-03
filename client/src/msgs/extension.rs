use crate::msgs::{
    array::{iter::ArrayIter, Array},
    enums::{ProtocolVersion, SignatureScheme},
    Codec, CodecSized, Decoder, Encoder,
};
use client::ClientExtension;

pub mod client;

#[macro_use]
mod macros;

#[derive(Debug)]
pub struct Extensions<'a, T: Codec<'a> + CodecSized<'a>>(Array<'a, T>);

impl<'a, T: Codec<'a> + CodecSized<'a>> Extensions<'a, T> {
    pub fn empty() -> Self {
        Self(Array::empty())
    }

    pub fn iter(&self) -> ArrayIter<'a, T> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Codec<'a> for Extensions<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        if self.0.is_empty() {
            return;
        }
        self.0.encode(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        Array::<'a, T>::decode(dec).map(Self)
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Default for Extensions<'a, T> {
    fn default() -> Self {
        Self::empty()
    }
}

impl<'a, T> From<T> for Extensions<'a, ClientExtension<'a>>
where
    T: Into<Array<'a, ClientExtension<'a>>>,
{
    fn from(data: T) -> Self {
        Self(data.into())
    }
}

ext_array!(SignatureSchemes, SignatureScheme);

// TODO: maybe should be part of ext_array! macro
impl<'a> CodecSized<'a> for SignatureSchemes<'a> {
    const HEADER_SIZE: usize = 2;

    fn data_size(&self) -> usize {
        self.0.data_size()
    }
}

ext_array!(ProtocolVersions, ProtocolVersion);

// TODO: maybe should be part of ext_array! macro
impl<'a> CodecSized<'a> for ProtocolVersions<'a> {
    const HEADER_SIZE: usize = 1;

    fn data_size(&self) -> usize {
        self.0.data_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgs::array::item::Item;
    use std::vec::Vec;

    mod encode {
        use super::*;

        #[test]
        fn empty_signature_schemes() {
            let schemes = SignatureSchemes::empty();
            let mut enc = Encoder::new(vec![]);
            schemes.encode(&mut enc);

            assert_eq!(schemes.data_size(), 0);
            assert_eq!(enc.bytes(), [0, 0]);
        }

        #[test]
        fn single_signature_scheme() {
            let schemes = SignatureSchemes::from(arr![SignatureScheme::RsaPkcs1Sha256]);
            let mut enc = Encoder::new(vec![]);
            schemes.encode(&mut enc);

            assert_eq!(schemes.data_size(), 2);
            assert_eq!(enc.bytes(), [0, 2, 0x04, 0x01]);
        }

        #[test]
        fn multiple_signature_schemes() {
            let schemes = SignatureSchemes::from(arr![
                SignatureScheme::RsaPkcs1Sha256,
                SignatureScheme::RsaPkcs1Sha384,
            ]);
            let mut enc = Encoder::new(vec![]);
            schemes.encode(&mut enc);

            assert_eq!(schemes.data_size(), 4);
            assert_eq!(enc.bytes(), [0, 4, 0x04, 0x01, 0x05, 0x01]);
        }

        #[test]
        fn empty_protocol_versions() {
            let versions = ProtocolVersions::empty();
            let mut enc = Encoder::new(vec![]);
            versions.encode(&mut enc);

            assert_eq!(versions.data_size(), 0);
            assert_eq!(enc.bytes(), [0]);
        }

        #[test]
        fn single_protocol_version() {
            let versions = ProtocolVersions::from(arr![ProtocolVersion::TLSv1_3]);
            let mut enc = Encoder::new(vec![]);
            versions.encode(&mut enc);

            assert_eq!(versions.data_size(), 2);
            assert_eq!(enc.bytes(), [2, 0x03, 0x04]);
        }

        #[test]
        fn multiple_protocol_versions() {
            let versions =
                ProtocolVersions::from(arr![ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2]);
            let mut enc = Encoder::new(vec![]);
            versions.encode(&mut enc);

            assert_eq!(versions.data_size(), 4);
            assert_eq!(enc.bytes(), [4, 0x03, 0x04, 0x03, 0x03]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn zero_length_signature_schemes() {
            let bytes = [0, 0];
            let mut dec = Decoder::new(&bytes);
            let schemes = SignatureSchemes::decode(&mut dec).unwrap();

            assert!(schemes.is_empty())
        }

        #[test]
        fn single_signature_scheme() {
            let bytes = [0, 2, 0x04, 0x01];
            let mut dec = Decoder::new(&bytes);
            let schemes = SignatureSchemes::decode(&mut dec).unwrap();

            assert_eq!(
                schemes.iter().collect::<Vec<Item<'_, SignatureScheme>>>(),
                vec![SignatureScheme::RsaPkcs1Sha256],
            );
        }

        #[test]
        fn multiple_signature_schemes() {
            let bytes = [0, 4, 0x04, 0x01, 0x05, 0x01];
            let mut dec = Decoder::new(&bytes);
            let schemes = SignatureSchemes::decode(&mut dec).unwrap();

            assert_eq!(
                schemes.iter().collect::<Vec<Item<'_, SignatureScheme>>>(),
                vec![
                    SignatureScheme::RsaPkcs1Sha256,
                    SignatureScheme::RsaPkcs1Sha384,
                ],
            );
        }

        #[test]
        fn zero_length_protocol_versions() {
            let bytes = [0];
            let mut dec = Decoder::new(&bytes);
            let versions = ProtocolVersions::decode(&mut dec).unwrap();

            assert!(versions.is_empty())
        }

        #[test]
        fn single_protocol_version() {
            let bytes = [2, 0x03, 0x04];
            let mut dec = Decoder::new(&bytes);
            let versions = ProtocolVersions::decode(&mut dec).unwrap();

            assert_eq!(
                versions.iter().collect::<Vec<Item<'_, ProtocolVersion>>>(),
                vec![ProtocolVersion::TLSv1_3],
            );
        }

        #[test]
        fn multiple_protocol_versions() {
            let bytes = [4, 0x03, 0x04, 0x03, 0x03];
            let mut dec = Decoder::new(&bytes);
            let versions = ProtocolVersions::decode(&mut dec).unwrap();

            assert_eq!(
                versions.iter().collect::<Vec<Item<'_, ProtocolVersion>>>(),
                vec![ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2],
            );
        }
    }
}
