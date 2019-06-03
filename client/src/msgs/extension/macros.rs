// TODO: Maybe should be called `ext_arr` to be consistent with the `arr` macro
macro_rules! ext_array {
    ($ident: ident, $header_size: expr, $ty: ty) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $ident<'a>(crate::msgs::array::Array<'a, $ty>);

        impl<'a> $ident<'a> {
            pub fn empty() -> Self {
                Self(Array::empty())
            }

            pub fn iter(&self) -> crate::msgs::array::iter::ArrayIter<'a, $ty> {
                self.0.iter()
            }

            pub fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<'a> crate::msgs::Codec<'a> for $ident<'a> {
            fn encode(&self, enc: &mut crate::msgs::Encoder<'a>) {
                self.encode_len(enc);
                self.0.encode_items(enc);
            }

            fn decode(dec: &mut crate::msgs::Decoder<'a>) -> Option<Self> {
                Self::decode_len(dec)
                    .and_then(|len| Array::decode_items(len, dec))
                    .map(Self)
            }
        }

        impl<'a> crate::msgs::CodecSized<'a> for $ident<'a> {
            const HEADER_SIZE: usize = $header_size;

            fn data_size(&self) -> usize {
                self.0.data_size()
            }
        }

        impl<'a> From<crate::msgs::array::Array<'a, $ty>> for $ident<'a> {
            fn from(data: crate::msgs::array::Array<'a, $ty>) -> Self {
                Self(data.into())
            }
        }
    };
}
