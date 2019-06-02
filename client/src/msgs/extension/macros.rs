// TODO: Maybe should be called `ext_arr` to be consistent with the `arr` macro
macro_rules! ext_array {
    ($ident: ident, $ty: ty) => {
        #[derive(Debug, Clone)]
        pub struct $ident<'a>(crate::msgs::array::Array<'a, $ty>);

        impl<'a> $ident<'a> {
            pub fn empty() -> Self {
                Self(Array::empty())
            }

            fn len(&self) -> usize {
                self.0.len()
            }

            fn is_empty(&self) -> bool {
                self.0.is_empty()
            }
        }

        impl<'a> crate::msgs::Codec<'a> for $ident<'a> {
            fn encode(&self, enc: &mut crate::msgs::Encoder<'a>) {
                self.encode_len(enc);
                for item in self.0.iter() {
                    item.encode(enc);
                }
            }

            fn decode(dec: &mut crate::msgs::Decoder<'a>) -> Option<Self> {
                crate::msgs::array::Array::decode(dec).map(Self)
            }
        }

        impl<'a> From<crate::msgs::array::Array<'a, $ty>> for $ident<'a> {
            fn from(data: crate::msgs::array::Array<'a, $ty>) -> Self {
                Self(data.into())
            }
        }
    };
}
