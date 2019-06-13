// TODO: Maybe should be called `ext_arr` to be consistent with the `arr` macro
macro_rules! ext_array {
    ($ident: ident, $header_size: expr, $ty: ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $ident<'a>(crate::array::Array<'a, $ty>);

        impl<'a> $ident<'a> {
            pub fn empty() -> Self {
                Self(Array::empty())
            }

            pub fn iter(&self) -> crate::array::iter::ArrayIter<'a, $ty> {
                self.0.iter()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<'a> crate::codec::Codec<'a> for $ident<'a> {
            fn encode(&self, enc: &mut crate::codec::encoder::Encoder<'a>) {
                self.encode_len(enc);
                self.0.encode_items(enc);
            }

            fn decode(dec: &mut crate::codec::decoder::Decoder<'a>) -> Option<Self> {
                Self::decode_len(dec)
                    .and_then(|len| Array::decode_items(len, dec))
                    .map(Self)
            }
        }

        impl<'a> crate::codec::CodecSized<'a> for $ident<'a> {
            const HEADER_SIZE: usize = $header_size;

            fn data_size(&self) -> usize {
                self.0.data_size()
            }
        }

        impl<'a> From<crate::array::Array<'a, $ty>> for $ident<'a> {
            fn from(data: crate::array::Array<'a, $ty>) -> Self {
                Self(data.into())
            }
        }
    };
}
