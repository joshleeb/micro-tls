use crate::{
    codec::{decoder::Decoder, encoder::Encoder, Codec},
    enums::{CipherSuite, CompressionMethod, ProtocolVersion},
    extension::{
        server::{ServerExtension, ServerRetryExtension},
        Extensions,
    },
    random::Random,
    session::SessionId,
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
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.server_version.encode(enc);
        self.random.encode(enc);
        self.session_id.encode(enc);
        self.cipher_suite.encode(enc);
        self.compression_method.encode(enc);
        self.extensions.encode(enc);
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

static SERVER_HELLO_RETRY_RANDOM: [u8; 32] = [
    0xcf, 0x21, 0xad, 0x74, 0xe5, 0x9a, 0x61, 0x11, 0xbe, 0x1d, 0x8c, 0x02, 0x1e, 0x65, 0xb8, 0x91,
    0xc2, 0xa2, 0x11, 0x16, 0x7a, 0xbb, 0x8c, 0x5e, 0x07, 0x9e, 0x09, 0xe2, 0xc8, 0xa8, 0x33, 0x9c,
];

#[derive(Debug, Default, PartialEq)]
pub struct ServerHelloRetryPayload<'a> {
    server_version: ProtocolVersion,
    session_id: SessionId,
    cipher_suite: CipherSuite,
    extensions: Extensions<'a, ServerRetryExtension>,
}

impl<'a> Codec<'a> for ServerHelloRetryPayload<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        let random = Random::from(SERVER_HELLO_RETRY_RANDOM);
        let compression_method = CompressionMethod::Null;

        self.server_version.encode(enc);
        random.encode(enc);
        self.session_id.encode(enc);
        self.cipher_suite.encode(enc);
        compression_method.encode(enc);
        self.extensions.encode_extensions(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let server_version = ProtocolVersion::decode(dec)?;
        let random = Random::decode(dec)?;
        let session_id = SessionId::decode(dec)?;
        let cipher_suite = CipherSuite::decode(dec)?;
        let compression_method = CompressionMethod::decode(dec)?;
        let extensions = Extensions::decode(dec)?;

        if compression_method != CompressionMethod::Null || random != SERVER_HELLO_RETRY_RANDOM {
            return None;
        }
        Some(ServerHelloRetryPayload {
            server_version,
            session_id,
            cipher_suite,
            extensions,
        })
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
                HelloRetryExtension as r_HelloRetryExtension,
                HelloRetryRequest as r_HelloRetryRequest, Random as r_Random,
                ServerExtension as r_ServerExtension, ServerHelloPayload as r_ServerHelloPayload,
                SessionID as r_SessionId,
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

        #[test]
        fn empty_retry() {
            assert_eq!(
                embed_bytes(ServerHelloRetryPayload::default()),
                rustls_bytes(r_HelloRetryRequest {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    session_id: r_SessionId::empty(),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn retry_session_id() {
            assert_eq!(
                embed_bytes(ServerHelloRetryPayload {
                    session_id: [97, 98, 99].into(),
                    ..Default::default()
                }),
                rustls_bytes(r_HelloRetryRequest {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    session_id: r_SessionId::new(&[97, 98, 99]),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn retry_extension_protocol_version() {
            assert_eq!(
                embed_bytes(ServerHelloRetryPayload {
                    extensions: Extensions::from(arr![ServerRetryExtension::from(
                        ProtocolVersion::TLSv1_3,
                    )]),
                    ..Default::default()
                }),
                rustls_bytes(r_HelloRetryRequest {
                    legacy_version: r_ProtocolVersion::TLSv1_2,
                    session_id: r_SessionId::empty(),
                    cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                    extensions: vec![r_HelloRetryExtension::SupportedVersions(
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

        #[test]
        fn empty_retry() {
            let bytes = rustls_bytes(r_HelloRetryRequest {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                session_id: r_SessionId::empty(),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloRetryPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.server_version, ProtocolVersion::TLSv1_2);
            assert!(payload.session_id.is_empty());
            assert_eq!(payload.cipher_suite, CipherSuite::TlsAes128GcmSha256);
            assert!(payload.extensions.is_empty());
        }

        #[test]
        fn retry_session_id() {
            let bytes = rustls_bytes(r_HelloRetryRequest {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                session_id: r_SessionId::new(&[97, 98, 99]),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloRetryPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.session_id, [97, 98, 99].into());
        }

        #[test]
        fn retry_extension_protocol_version() {
            let bytes = rustls_bytes(r_HelloRetryRequest {
                legacy_version: r_ProtocolVersion::TLSv1_2,
                session_id: r_SessionId::new(&[]),
                cipher_suite: r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                extensions: vec![r_HelloRetryExtension::SupportedVersions(
                    r_ProtocolVersion::TLSv1_3,
                )],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ServerHelloRetryPayload::decode(&mut dec).unwrap();

            assert_eq!(
                payload.extensions,
                Extensions::from(arr![ServerRetryExtension::from(ProtocolVersion::TLSv1_3)]),
            );
        }
    }

    fn rustls_bytes<T: r_Codec>(payload: T) -> Vec<u8> {
        let mut enc = vec![];
        payload.encode(&mut enc);
        enc
    }

    fn embed_bytes<'a, T: Codec<'a>>(payload: T) -> Vec<u8> {
        let mut enc = Encoder::new(vec![]);
        payload.encode(&mut enc);
        enc.bytes().into()
    }
}
