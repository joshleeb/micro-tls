use crate::{
    codec::{Codec, CodecSized, Decoder, Encoder, HeaderSize},
    handshake::enums::{ExtensionType, ProtocolVersion},
};

// TODO: Add unknown server extension
#[derive(Debug, PartialEq)]
pub enum ServerExtension {
    SupportedVersions(ProtocolVersion),
}

impl ServerExtension {
    fn ty(&self) -> ExtensionType {
        match self {
            ServerExtension::SupportedVersions(_) => ExtensionType::SupportedVersions,
        }
    }

    // TODO: Document this.
    fn ext_size(&self) -> usize {
        match self {
            ServerExtension::SupportedVersions(ref r) => r.data_size(),
        }
    }
}

impl<'a> Codec<'a> for ServerExtension {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.ty().encode(enc);

        // TODO: Document this, and use a nicer method (perhaps part of CodecSized).
        (self.ext_size() as u16).encode(enc);

        match self {
            ServerExtension::SupportedVersions(ref r) => r.encode(enc),
        };
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let ty = ExtensionType::decode(dec)?;
        let len = Self::decode_len(dec)?;
        let mut sub = dec.sub(len)?;

        match ty {
            ExtensionType::SupportedVersions => {
                ProtocolVersion::decode(&mut sub).map(ServerExtension::from)
            }
            // TODO: Handle unknown server extension type
            ExtensionType::Unknown(_) | _ => unimplemented!(),
        }
    }
}

impl<'a> CodecSized<'a> for ServerExtension {
    const HEADER_SIZE: HeaderSize = HeaderSize::U16;

    fn data_size(&self) -> usize {
        Self::HEADER_SIZE.size() + self.ty().data_size() + self.ext_size()
    }
}

impl From<ProtocolVersion> for ServerExtension {
    fn from(data: ProtocolVersion) -> Self {
        ServerExtension::SupportedVersions(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod encode {
        use super::*;

        #[test]
        fn supported_versions() {
            let ext = ServerExtension::from(ProtocolVersion::TLSv1_2);
            let mut enc = Encoder::new(vec![]);
            ext.encode(&mut enc);

            assert_eq!(ext.data_size(), 6);
            assert_eq!(enc.bytes(), [0x00, 0x2b, 0, 2, 3, 3]);
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn supported_versions() {
            let bytes = [0x00, 0x2b, 0, 2, 3, 3];
            let mut dec = Decoder::new(&bytes);

            assert_eq!(
                ServerExtension::decode(&mut dec).unwrap(),
                ServerExtension::from(ProtocolVersion::TLSv1_2),
            );
        }
    }
}
