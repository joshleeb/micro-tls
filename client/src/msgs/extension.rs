use crate::msgs::{
    array::Array,
    enums::{ProtocolVersion, SignatureScheme},
    Codec, CodecSized, Decoder, Encoder,
};
use client::ClientExtension;

pub mod client;

#[derive(Debug)]
pub struct Extensions<'a, T: Codec<'a> + CodecSized<'a>> {
    inner: Array<'a, T>,
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Extensions<'a, T> {
    pub fn empty() -> Self {
        Self {
            inner: Array::empty(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<'a, T: Codec<'a> + CodecSized<'a>> Codec<'a> for Extensions<'a, T> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        if self.inner.is_empty() {
            return;
        }
        self.inner.encode(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        Array::<'a, T>::decode(dec).map(|inner| Extensions { inner })
    }
}

impl<'a, T> From<T> for Extensions<'a, ClientExtension<'a>>
where
    T: Into<Array<'a, ClientExtension<'a>>>,
{
    fn from(inner: T) -> Self {
        Self {
            inner: inner.into(),
        }
    }
}

macro_rules! ext_array {
    ($ident: ident, $ty: ty) => {
        #[derive(Debug, Clone)]
        pub struct $ident<'a> {
            pub inner: crate::msgs::array::Array<'a, $ty>,
        }

        impl<'a> $ident<'a> {
            pub fn empty() -> Self {
                Self {
                    inner: Array::empty(),
                }
            }

            fn len(&self) -> usize {
                self.inner.len()
            }

            fn is_empty(&self) -> bool {
                self.inner.is_empty()
            }
        }

        impl<'a> crate::msgs::Codec<'a> for $ident<'a> {
            fn encode(&self, enc: &mut crate::msgs::Encoder<'a>) {
                self.encode_len(enc);
                for item in self.inner.iter() {
                    item.encode(enc);
                }
            }

            fn decode(dec: &mut crate::msgs::Decoder<'a>) -> Option<Self> {
                crate::msgs::array::Array::decode(dec).map(|inner| Self { inner })
            }
        }

        impl<'a, T> From<T> for Extensions<'a, $ty>
        where
            T: Into<crate::msgs::array::Array<'a, $ty>>,
        {
            fn from(inner: T) -> Self {
                Self {
                    inner: inner.into(),
                }
            }
        }
    };
}

ext_array!(SignatureSchemes, SignatureScheme);

impl<'a> CodecSized<'a> for SignatureSchemes<'a> {
    const HEADER_SIZE: usize = 2;

    fn data_size(&self) -> usize {
        self.inner.data_size()
    }
}

ext_array!(ProtocolVersions, ProtocolVersion);

impl<'a> CodecSized<'a> for ProtocolVersions<'a> {
    const HEADER_SIZE: usize = 1;

    fn data_size(&self) -> usize {
        self.inner.data_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn empty_signautre_schemes() {
            let arr = SignatureSchemes::empty();
            let mut enc = Encoder::new(vec![]);
            arr.encode(&mut enc);

            assert_eq!(arr.data_size(), 0);
            assert_eq!(enc.bytes(), [0, 0]);
        }

        #[test]
        fn single_signature_scheme() {
            let arr = SignatureSchemes {
                inner: Array::from([SignatureScheme::RSA_PKCS1_SHA256].as_ref()),
            };
            let mut enc = Encoder::new(vec![]);
            arr.encode(&mut enc);

            assert_eq!(arr.data_size(), 2);
            assert_eq!(enc.bytes(), [0, 2, 0x04, 0x01]);
        }

        #[test]
        fn multiple_signature_schemes() {
            let arr = SignatureSchemes {
                inner: Array::from(
                    [
                        SignatureScheme::RSA_PKCS1_SHA256,
                        SignatureScheme::RSA_PKCS1_SHA384,
                    ]
                    .as_ref(),
                ),
            };
            let mut enc = Encoder::new(vec![]);
            arr.encode(&mut enc);

            assert_eq!(arr.data_size(), 4);
            assert_eq!(enc.bytes(), [0, 4, 0x04, 0x01, 0x05, 0x01]);
        }

        #[test]
        fn empty_protocol_versions() {
            let arr = ProtocolVersions::empty();
            let mut enc = Encoder::new(vec![]);
            arr.encode(&mut enc);

            assert_eq!(arr.data_size(), 0);
            assert_eq!(enc.bytes(), [0]);
        }

        #[test]
        fn single_protocol_version() {
            let arr = ProtocolVersions {
                inner: Array::from([ProtocolVersion::TLSv1_3].as_ref()),
            };
            let mut enc = Encoder::new(vec![]);
            arr.encode(&mut enc);

            assert_eq!(arr.data_size(), 2);
            assert_eq!(enc.bytes(), [2, 0x03, 0x04]);
        }

        #[test]
        fn multiple_protocol_versions() {
            let arr = ProtocolVersions {
                inner: Array::from([ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2].as_ref()),
            };
            let mut enc = Encoder::new(vec![]);
            arr.encode(&mut enc);

            assert_eq!(arr.data_size(), 4);
            assert_eq!(enc.bytes(), [4, 0x03, 0x04, 0x03, 0x03]);
        }
    }

    mod decode {
        use super::*;
    }
}
