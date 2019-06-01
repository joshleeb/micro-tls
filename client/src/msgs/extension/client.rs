use crate::msgs::{
    enums::ExtensionType,
    extension::{ProtocolVersions, SignatureSchemes},
    Codec, CodecSized, Decoder, Encoder,
};

#[derive(Debug)]
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
    pub fn ext_size(&self) -> usize {
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
                SignatureSchemes::decode(&mut sub).map(ClientExtension::SignatureAlgorithms)
            }
            ExtensionType::SupportedVersions => {
                ProtocolVersions::decode(&mut sub).map(ClientExtension::SupportedVersions)
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgs::extension::{ProtocolVersions, SignatureSchemes};

    mod encode {
        use super::*;

        #[test]
        fn empty_signature_algorithms() {
            let ext = ClientExtension::SignatureAlgorithms(SignatureSchemes::empty());
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 6);
            assert_eq!(enc.bytes(), [0x00, 0x0d, 0, 2, 0, 0]);
        }

        #[test]
        fn empty_supported_versions() {
            let ext = ClientExtension::SupportedVersions(ProtocolVersions::empty());
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 5);
            assert_eq!(enc.bytes(), [0x00, 0x2b, 0, 1, 0]);
        }
    }

    mod decode {
        use super::*;
    }
}
