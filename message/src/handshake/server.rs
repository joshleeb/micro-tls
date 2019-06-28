use crate::{
    codec::{Codec, CodecSized, Decoder, Encoder, HeaderSize},
    error::Result as TlsResult,
    handshake::{
        enums::{CipherSuite, CompressionMethod, ProtocolVersion},
        extension::{server::ServerExtension, Extensions},
        random::Random,
        session::SessionId,
    },
};

#[derive(Debug, Default, PartialEq)]
pub struct ServerHelloPayload<'a> {
    server_version: ProtocolVersion,
    random: Random,
    session_id: SessionId,
    cipher_suite: CipherSuite,
    compression_method: CompressionMethod,
    extensions: Extensions<'a, ServerExtension>,
}

impl<'a> Codec<'a> for ServerHelloPayload<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) -> TlsResult<()> {
        self.server_version.encode(enc)?;
        self.random.encode(enc)?;
        self.session_id.encode(enc)?;
        self.cipher_suite.encode(enc);
        self.compression_method.encode(enc)?;
        self.extensions.encode(enc)
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        Some(ServerHelloPayload {
            server_version: ProtocolVersion::decode(dec)?,
            random: Random::decode(dec)?,
            session_id: SessionId::decode(dec)?,
            cipher_suite: CipherSuite::decode(dec)?,
            compression_method: CompressionMethod::decode(dec)?,
            extensions: Extensions::decode(dec)?,
        })
    }
}

impl<'a> CodecSized<'a> for ServerHelloPayload<'a> {
    const HEADER_SIZE: HeaderSize = HeaderSize::U24;

    fn data_size(&self) -> usize {
        self.server_version.data_size()
            + self.random.data_size()
            + self.session_id.data_size()
            + self.cipher_suite.data_size()
            + self.compression_method.data_size()
            + self.extensions.data_size()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rustls::{
        internal::msgs::{
            codec::Codec as r_Codec,
            enums::Compression as r_Compression,
            handshake::{
                Random as r_Random, ServerExtension as r_ServerExtension,
                ServerHelloPayload as r_ServerHelloPayload, SessionID as r_SessionId,
            },
        },
        CipherSuite as r_CipherSuite, ProtocolVersion as r_ProtocolVersion,
    };
    use std::vec::Vec;

    mod encode {
        use super::*;

        #[test]
        fn empty_hello() {
            assert_eq!(
                embed_bytes(ServerHelloPayload::default()),
                rustls_bytes(r_ServerHelloPayload {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    compression_method: r_Compression::Null,
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn hello_random() {
            assert_eq!(
                embed_bytes(ServerHelloPayload {
                    random: [9; 32].into(),
                    ..Default::default()
                }),
                rustls_bytes(r_ServerHelloPayload {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[9; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    compression_method: r_Compression::Null,
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn hello_session_id() {
            assert_eq!(
                embed_bytes(ServerHelloPayload {
                    session_id: [97, 98, 99].into(),
                    ..Default::default()
                }),
                rustls_bytes(r_ServerHelloPayload {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::new(&[97, 98, 99]),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    compression_method: r_Compression::Null,
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn hello_extension_protocol_version() {
            assert_eq!(
                embed_bytes(ServerHelloPayload {
                    extensions: Extensions::from(arr![ServerExtension::from(
                        ProtocolVersion::TLSv1_3,
                    )]),
                    ..Default::default()
                }),
                rustls_bytes(r_ServerHelloPayload {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    compression_method: r_Compression::Null,
                    extensions: vec![r_ServerExtension::SupportedVersions(
                        r_ProtocolVersion::TLSv1_3,
                    )],
                }),
            )
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty_hello() {
            let bytes = rustls_bytes(r_ServerHelloPayload {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                random: r_Random::from_slice(&[0; 32]),
                session_id: r_SessionId::empty(),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                compression_method: r_Compression::Null,
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.server_version, ProtocolVersion::TLSv1_2);
            assert_eq!(payload.random, Random::default());
            assert!(payload.session_id.is_empty());
            assert_eq!(payload.cipher_suite, CipherSuite::TlsAes128GcmSha256);
            assert_eq!(payload.compression_method, CompressionMethod::Null);
            assert!(payload.extensions.is_empty());
        }

        #[test]
        fn hello_random() {
            let bytes = rustls_bytes(r_ServerHelloPayload {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                random: r_Random::from_slice(&[9; 32]),
                session_id: r_SessionId::empty(),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                compression_method: r_Compression::Null,
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.random, Random::from([9; 32]));
        }

        #[test]
        fn hello_session_id() {
            let bytes = rustls_bytes(r_ServerHelloPayload {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                random: r_Random::from_slice(&[0; 32]),
                session_id: r_SessionId::new(&[97, 98, 99]),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                compression_method: r_Compression::Null,
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.session_id, [97, 98, 99].into());
        }

        #[test]
        fn hello_extension_protocol_version() {
            let bytes = rustls_bytes(r_ServerHelloPayload {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                random: r_Random::from_slice(&[0; 32]),
                session_id: r_SessionId::new(&[]),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                compression_method: r_Compression::Null,
                extensions: vec![r_ServerExtension::SupportedVersions(
                    r_ProtocolVersion::TLSv1_3,
                )],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloPayload::decode(&mut dec).unwrap();

            assert_eq!(
                payload.extensions,
                Extensions::from(arr![ServerExtension::from(ProtocolVersion::TLSv1_3)]),
            );
        }
    }

    fn rustls_bytes<T: r_Codec>(payload: T) -> Vec<u8> {
        let mut enc = vec![];
        payload.encode(&mut enc);
        enc
    }

    fn embed_bytes<'a, T: CodecSized<'a>>(payload: T) -> Vec<u8> {
        let mut enc = Encoder::new(vec![]);
        payload.encode(&mut enc).unwrap();
        assert_eq!(enc.bytes().len(), payload.data_size());

        enc.bytes().into()
    }
}
