use crate::msgs::{
    array::{iter::ArrayIter, Array},
    enums::ExtensionType,
    enums::ProtocolVersion,
    Codec, CodecLength, Decoder, Encoder,
};
use core::u16;

macro_rules! exts_enum {
    { $($var: ident => $ty: ty),*$(,)? } => {
        pub enum Extension<'a> {
            $($var(Array<'a, $ty>)),*,
        }

        impl<'a> Extension<'a> {
            fn is_empty(&self) -> bool {
                match self {
                    $(Extension::$var(data) => data.is_empty()),*,
                }
            }

            fn ty(&self) -> ExtensionType {
                match self {
                    $(Extension::$var(_) => ExtensionType::$var),*,
                }
            }
        }

        impl<'a> Codec<'a> for Extension<'a> {
            fn encode(&self, enc: &mut Encoder<'a>) {
                if self.is_empty() {
                    return;
                }

                self.ty().encode(enc);
                match self {
                    $(Extension::$var(data) => data.encode(enc)),*,
                };
            }

            fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
                let ty = ExtensionType::decode(dec)?;
                match ty {
                    $(ExtensionType::$var => {
                        Array::<$ty>::decode(dec).map(|data| Extension::$var(data))
                    }),*,
                    ExtensionType::Unknown(_) => unimplemented!(),
                }
            }
        }

        $(
            impl<'a> Extension<'a> {
                fn iter(&self) -> ArrayIter<'a, $ty> {
                    if let Extension::$var(data) = self {
                        return data.iter();
                    }
                    unreachable!()
                }
            }

            impl<'a> From<Array<'a, $ty>> for Extension<'a> {
                fn from(data: Array<'a, $ty>) -> Self {
                    Extension::$var(data)
                }
            }
        )*
    };
}

exts_enum! {
    SupportedVersions => ProtocolVersion,
}

impl<'a> CodecLength<'a> for Extension<'a> {
    const LENGTH: usize = 2;

    fn encode_len(len: usize, enc: &mut Encoder<'a>) {
        debug_assert!(len <= usize::from(u16::MAX));
        (len as u16).encode(enc);
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        u16::decode(dec).map(usize::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgs::enums::ProtocolVersion;
    use std::vec::Vec;

    #[test]
    fn empty_len() {
        let extension = Extension::from(Array::<ProtocolVersion>::empty());
        assert!(extension.is_empty());
    }

    #[test]
    fn extension_type() {
        let extension = Extension::from(Array::<ProtocolVersion>::empty());
        assert_eq!(extension.ty(), ExtensionType::SupportedVersions);
    }

    mod encode {
        use super::*;

        #[test]
        fn empty() {
            let extension = Extension::from(Array::<ProtocolVersion>::empty());
            let mut enc = Encoder::new(vec![]);
            extension.encode(&mut enc);

            assert!(enc.bytes().is_empty());
        }

        #[test]
        fn single_data_items() {
            let data = Array::from([ProtocolVersion::TLSv1_2].as_ref());
            let extension = Extension::from(data.clone());
            let mut enc = Encoder::new(vec![]);
            extension.encode(&mut enc);

            let mut expected_enc = Encoder::new(vec![]);
            ExtensionType::SupportedVersions.encode(&mut expected_enc);
            data.encode(&mut expected_enc);

            assert_eq!(enc.bytes(), expected_enc.bytes());
        }

        #[test]
        fn multiple_data_items() {
            let data = Array::from([ProtocolVersion::TLSv1_2, ProtocolVersion::TLSv1_3].as_ref());
            let extension = Extension::from(data.clone());
            let mut enc = Encoder::new(vec![]);
            extension.encode(&mut enc);

            let mut expected_enc = Encoder::new(vec![]);
            ExtensionType::SupportedVersions.encode(&mut expected_enc);
            data.encode(&mut expected_enc);

            assert_eq!(enc.bytes(), expected_enc.bytes());
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty() {
            let bytes = [];
            let mut dec = Decoder::new(&bytes);
            let extension = Extension::decode(&mut dec);

            assert!(extension.is_none());
        }

        #[test]
        fn single_bytes() {
            let data = Array::from([ProtocolVersion::TLSv1_2].as_ref());
            let mut enc = Encoder::new(vec![]);
            ExtensionType::SupportedVersions.encode(&mut enc);
            data.encode(&mut enc);

            let mut dec = Decoder::new(enc.bytes());
            let extension = Extension::decode(&mut dec).unwrap();

            assert_eq!(extension.ty(), ExtensionType::SupportedVersions);
            assert_eq!(
                extension.iter().collect::<Vec<ProtocolVersion>>(),
                data.iter().collect::<Vec<ProtocolVersion>>()
            );
        }

        #[test]
        fn multiple_bytes() {
            let data = Array::from([ProtocolVersion::TLSv1_2, ProtocolVersion::TLSv1_3].as_ref());
            let mut enc = Encoder::new(vec![]);
            ExtensionType::SupportedVersions.encode(&mut enc);
            data.encode(&mut enc);

            let mut dec = Decoder::new(enc.bytes());
            let extension = Extension::decode(&mut dec).unwrap();

            assert_eq!(extension.ty(), ExtensionType::SupportedVersions);
            assert_eq!(
                extension.iter().collect::<Vec<ProtocolVersion>>(),
                data.iter().collect::<Vec<ProtocolVersion>>()
            );
        }
    }
}
