use crate::msgs::{
    array::Array,
    enums::{CipherSuite, CompressionMethod, ExtensionType, ProtocolVersion},
    random::Random,
    session::SessionId,
    Codec, Decoder, Encoder,
};

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
    pub cipher_suites: Array<'a, CipherSuite>,
    pub legacy_compression_methods: Array<'a, CompressionMethod>,
    pub extensions: Array<'a, ExtensionType>,
}

impl<'a> Codec<'a> for ClientHelloPayload<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.legacy_version.encode(enc);
        self.random.encode(enc);
        self.legacy_session_id.encode(enc);
        self.cipher_suites.encode(enc);
        self.legacy_compression_methods.encode(enc);
        self.extensions.encode(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        Some(ClientHelloPayload {
            legacy_version: ProtocolVersion::decode(dec)?,
            random: Random::decode(dec)?,
            legacy_session_id: SessionId::decode(dec)?,
            cipher_suites: Array::decode(dec)?,
            legacy_compression_methods: Array::decode(dec)?,
            extensions: Array::decode(dec)?,
        })
    }
}
