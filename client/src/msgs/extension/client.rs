use crate::msgs::{
    enums::ExtensionType,
    extension::{ProtocolVersions, SignatureSchemes},
    Codec, CodecLength, Decoder, Encoder,
};
use core::u16;

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
}

impl<'a> Codec<'a> for ClientExtension<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.ty().encode(enc);

        match self {
            ClientExtension::SignatureAlgorithms(ref r) => {
                r.encode(enc);
            }
            ClientExtension::SupportedVersions(ref r) => {
                r.encode(enc);
            }
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

impl<'a> CodecLength<'a> for ClientExtension<'a> {
    const LENGTH: usize = 2;

    fn encode_len(len: usize, enc: &mut Encoder<'a>) {
        debug_assert!(len <= usize::from(u16::MAX));
        (len as u16).encode(enc);
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        u16::decode(dec).map(usize::from)
    }
}
