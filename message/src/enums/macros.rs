macro_rules! msg_enum {
    (
        $ident: ident, $ty: ty;
        { $($var: ident => $val: expr),*$(,)? }
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum $ident {
            $($var),*,
            Unknown($ty),
        }

        impl From<$ident> for $ty {
            fn from(val: $ident) -> Self {
                match val {
                    $($ident::$var => $val),*,
                    $ident::Unknown(x) => x,
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

        impl<'a> crate::codec::Codec<'a> for $ident {
            fn encode(&self, enc: &mut crate::codec::encoder::Encoder<'a>) {
                <$ty>::from(*self).encode(enc);
            }

            fn decode(dec: &mut crate::codec::decoder::Decoder<'a>) -> Option<Self> {
                <$ty>::decode(dec).map(|item| {
                    match item {
                        $($val => $ident::$var),*,
                        x => $ident::Unknown(x),
                    }
                })
            }
        }

        impl<'a> crate::codec::CodecSized<'a> for $ident {
            const HEADER_SIZE: usize = <$ty>::HEADER_SIZE;

            fn data_size(&self) -> usize {
                <$ty>::data_size(&0)
            }
        }
    };
}

macro_rules! enum_default {
    ($ident: ident, $var: ident) => {
        impl Default for $ident {
            fn default() -> Self {
                $ident::$var
            }
        }
    };
}
