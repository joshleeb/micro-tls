use crate::msgs::{
    array::Array,
    enums::{ProtocolVersion, SignatureScheme},
    Codec, CodecLength, Decoder, Encoder,
};
use client::ClientExtension;

pub mod client;

#[derive(Debug)]
pub struct Extensions<'a, T: Codec<'a> + CodecLength<'a>> {
    inner: Array<'a, T>,
}

impl<'a, T: Codec<'a> + CodecLength<'a>> Extensions<'a, T> {
    pub fn empty() -> Self {
        Self {
            inner: Array::empty(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl<'a, T> Codec<'a> for Extensions<'a, T>
where
    T: Codec<'a> + CodecLength<'a>,
{
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
                self.inner.encode(enc);
            }

            fn decode(dec: &mut crate::msgs::Decoder<'a>) -> Option<Self> {
                crate::msgs::array::Array::decode(dec).map(|inner| Self { inner })
            }
        }

        impl<'a> crate::msgs::CodecLength<'a> for $ident<'a> {
            const LENGTH: usize = <$ty>::LENGTH;

            fn encode_len(len: usize, enc: &mut crate::msgs::Encoder<'a>) {
                <$ty>::encode_len(len, enc);
            }

            fn decode_len(dec: &mut crate::msgs::Decoder<'a>) -> Option<usize> {
                <$ty>::decode_len(dec)
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

ext_array!(ProtocolVersions, ProtocolVersion);
ext_array!(SignatureSchemes, SignatureScheme);
