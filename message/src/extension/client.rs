use crate::{
    array::Array,
    codec::{decoder::Decoder, encoder::Encoder, Codec, CodecSized},
    enums::{ExtensionType, ProtocolVersion, SignatureScheme},
    extension::{ProtocolVersions, SignatureSchemes},
};

// TODO: Add unknown client extension
#[derive(Debug, PartialEq)]
pub enum ClientExtension<'a> {
    SignatureAlgorithms(SignatureSchemes<'a>),
    SupportedVersions(ProtocolVersions<'a>),
}

impl<'a> ClientExtension<'a> {
    pub fn ty(&self) -> ExtensionType {
        match self {
            ClientExtension::SignatureAlgorithms(_) => ExtensionType::SignatureAlgorithms,
            ClientExtension::SupportedVersions(_) => ExtensionType::SupportedVersions,
        }
    }

    // TODO: Document this.
    fn ext_size(&self) -> usize {
        match self {
            ClientExtension::SignatureAlgorithms(ref r) => {
                SignatureSchemes::HEADER_SIZE + r.data_size()
            }
            ClientExtension::SupportedVersions(ref r) => {
                ProtocolVersions::HEADER_SIZE + r.data_size()
            }
        }
    }
}

impl<'a> Codec<'a> for ClientExtension<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.ty().encode(enc);

        // TODO: Document this, and use a nicer method (perhaps part of CodecSized).
        (self.ext_size() as u16).encode(enc);

        match self {
            ClientExtension::SignatureAlgorithms(ref r) => r.encode(enc),
            ClientExtension::SupportedVersions(ref r) => r.encode(enc),
        };
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let ty = ExtensionType::decode(dec)?;
        let len = Self::decode_len(dec)?;
        let mut sub = dec.sub(len)?;

        match ty {
            ExtensionType::SignatureAlgorithms => {
                SignatureSchemes::decode(&mut sub).map(ClientExtension::from)
            }
            ExtensionType::SupportedVersions => {
                ProtocolVersions::decode(&mut sub).map(ClientExtension::from)
            }
            // TODO: Handle unknown client extension type
            ExtensionType::Unknown(_) => unimplemented!(),
        }
    }
}

impl<'a> CodecSized<'a> for ClientExtension<'a> {
    const HEADER_SIZE: usize = 2;

    fn data_size(&self) -> usize {
        Self::HEADER_SIZE + self.ty().data_size() + self.ext_size()
    }
}

impl<'a> From<Array<'a, SignatureScheme>> for ClientExtension<'a> {
    fn from(data: Array<'a, SignatureScheme>) -> Self {
        ClientExtension::from(SignatureSchemes::from(data))
    }
}

impl<'a> From<SignatureSchemes<'a>> for ClientExtension<'a> {
    fn from(data: SignatureSchemes<'a>) -> Self {
        ClientExtension::SignatureAlgorithms(data)
    }
}

impl<'a> From<Array<'a, ProtocolVersion>> for ClientExtension<'a> {
    fn from(data: Array<'a, ProtocolVersion>) -> Self {
        ClientExtension::from(ProtocolVersions::from(data))
    }
}

impl<'a> From<ProtocolVersions<'a>> for ClientExtension<'a> {
    fn from(data: ProtocolVersions<'a>) -> Self {
        ClientExtension::SupportedVersions(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::extension::{ProtocolVersions, SignatureSchemes};

    mod encode {
        use super::*;

        #[test]
        fn empty_signature_algorithms() {
            let ext = ClientExtension::from(SignatureSchemes::empty());
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 6);
            assert_eq!(enc.bytes(), [0x00, 0x0d, 0, 2, 0, 0]);
        }

        #[test]
        fn single_signature_algorithm() {
            let ext = ClientExtension::from(arr![SignatureScheme::RsaPkcs1Sha256]);
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 8);
            assert_eq!(enc.bytes(), [0x00, 0x0d, 0, 4, 0, 2, 4, 1]);
        }

        #[test]
        fn multiple_signature_algorithms() {
            let ext = ClientExtension::from(arr![
                SignatureScheme::RsaPkcs1Sha256,
                SignatureScheme::EcdsaNistp256Sha256,
            ]);
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 10);
            assert_eq!(enc.bytes(), [0x00, 0x0d, 0, 6, 0, 4, 4, 1, 4, 3]);
        }

        #[test]
        fn empty_supported_versions() {
            let ext = ClientExtension::from(ProtocolVersions::empty());
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 5);
            assert_eq!(enc.bytes(), [0x00, 0x2b, 0, 1, 0]);
        }

        #[test]
        fn single_protocol_version() {
            let ext = ClientExtension::from(arr![ProtocolVersion::TLSv1_2]);
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 7);
            assert_eq!(enc.bytes(), [0x00, 0x2b, 0, 3, 2, 3, 3]);
        }

        #[test]
        fn multiple_protocol_versions() {
            let ext =
                ClientExtension::from(arr![ProtocolVersion::TLSv1_2, ProtocolVersion::TLSv1_3]);
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 9);
            assert_eq!(enc.bytes(), [0x00, 0x2b, 0, 5, 4, 3, 3, 3, 4]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty_signature_algorithms() {
            let bytes = [0x00, 0x0d, 0, 2, 0, 0];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ClientExtension::decode(&mut dec).unwrap(),
                ClientExtension::from(SignatureSchemes::empty()),
            );
        }

        #[test]
        fn single_signature_algorithm() {
            let bytes = [0x00, 0x0d, 0, 4, 0, 2, 4, 1];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ClientExtension::decode(&mut dec).unwrap(),
                ClientExtension::from(arr![SignatureScheme::RsaPkcs1Sha256]),
            );
        }

        #[test]
        fn multiple_signature_algorithms() {
            let bytes = [0x00, 0x0d, 0, 6, 0, 4, 4, 1, 4, 3];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ClientExtension::decode(&mut dec).unwrap(),
                ClientExtension::from(arr![
                    SignatureScheme::RsaPkcs1Sha256,
                    SignatureScheme::EcdsaNistp256Sha256,
                ]),
            );
        }

        #[test]
        fn empty_supported_versions() {
            let bytes = [0x00, 0x2b, 0, 1, 0];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ClientExtension::decode(&mut dec).unwrap(),
                ClientExtension::from(ProtocolVersions::empty()),
            );
        }

        #[test]
        fn single_supported_versions() {
            let bytes = [0x00, 0x2b, 0, 3, 2, 3, 3];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ClientExtension::decode(&mut dec).unwrap(),
                ClientExtension::from(arr![ProtocolVersion::TLSv1_2]),
            );
        }

        #[test]
        fn multiple_supported_versions() {
            let bytes = [0x00, 0x2b, 0, 5, 4, 3, 3, 3, 4];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ClientExtension::decode(&mut dec).unwrap(),
                ClientExtension::from(arr![ProtocolVersion::TLSv1_2, ProtocolVersion::TLSv1_3,]),
            );
        }
    }
}
