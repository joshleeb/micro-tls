use crate::msgs::{
    self,
    enums::{CipherSuite, CompressionMethod, ExtensionType, ProtocolVersion},
    random::Random,
    session::SessionId,
    Codec, Decoder, Encoder,
};
use managed::ManagedSlice;

// TODO: HandshakePayload should have an Unknown(Payload) variant
pub enum HandshakePayload<'a> {
    ClientHello(ClientHelloPayload<'a>),
}

impl<'a> HandshakePayload<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        match self {
            HandshakePayload::ClientHello(ref x) => x.encode(enc),
        };
    }
}

pub struct ClientHelloPayload<'a> {
    pub legacy_version: ProtocolVersion,
    pub random: Random,
    pub legacy_session_id: SessionId,
    pub cipher_suites: ManagedSlice<'a, CipherSuite>,
    pub legacy_compression_methods: ManagedSlice<'a, CompressionMethod>,
    pub extensions: ManagedSlice<'a, ExtensionType>,
}

impl<'a> Codec<'a> for ClientHelloPayload<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.legacy_version.encode(enc);
        self.random.encode(enc);
        self.legacy_session_id.encode(enc);
        self.cipher_suites.encode(enc);
        self.legacy_compression_methods.encode(enc);

        if !self.extensions.is_empty() {
            self.extensions.encode(enc);
        }
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let mut payload = ClientHelloPayload {
            legacy_version: ProtocolVersion::decode(dec)?,
            random: Random::decode(dec)?,
            legacy_session_id: SessionId::decode(dec)?,
            cipher_suites: ManagedSlice::decode(dec)?,
            legacy_compression_methods: ManagedSlice::decode(dec)?,
            extensions: [].into(),
        };

        if !dec.is_complete() {
            payload.extensions = ManagedSlice::decode(dec)?;
        }
        Some(payload)
    }
}
