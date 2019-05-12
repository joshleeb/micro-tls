macro_rules! msg_enum {
    (
        $ident: ident, $ty: ty;
        { $($var: ident => $val: expr),*$(,)? }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $ident {
            $( $var),*,
            Unknown($ty),
            _None,
        }

        impl From<$ident> for $ty {
            fn from(val: $ident) -> Self {
                match val {
                    $($ident::$var => $val),*,
                    $ident::Unknown(x) => x,
                    $ident::_None => unreachable!(),
                }
            }
        }

        impl From<$ty> for $ident {
            fn from(val: $ty) -> Self {
                match val {
                    $($val => $ident::$var),*,
                    x => $ident::Unknown(x),
                }
            }
        }

        impl<'a> crate::msgs::Codec<'a> for $ident {
            fn encode(&self, enc: &mut crate::msgs::Encoder<'a>) {
                <$ty>::from(*self).encode(enc);
            }

            fn decode(dec: &mut crate::msgs::Decoder<'a>) -> Option<Self> {
                <$ty>::decode(dec).map(|item| {
                    match item {
                        $($val => $ident::$var),*,
                        x => $ident::Unknown(x),
                    }
                })
            }
        }

        impl<'a> crate::msgs::CodecLength<'a> for $ident {
            const LENGTH: usize = <$ty>::LENGTH;

            fn encode_len(len: usize, enc: &mut crate::msgs::Encoder<'a>) {
                <$ty>::encode_len(len, enc);
            }

            fn decode_len(dec: &mut crate::msgs::Decoder<'a>) -> Option<usize> {
                <$ty>::decode_len(dec)
            }
        }
    };
}
