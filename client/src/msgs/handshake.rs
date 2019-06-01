use crate::msgs::{
    array::Array,
    enums::{CipherSuite, CompressionMethod, ProtocolVersion},
    extension::{client::ClientExtension, Extensions},
    random::Random,
    session::SessionId,
    Codec, CodecSized, Decoder, Encoder,
};

#[derive(Debug)]
pub struct ClientHelloPayload<'a> {
    pub client_version: ProtocolVersion,
    pub random: Random,
    pub session_id: SessionId,
    pub cipher_suites: Array<'a, CipherSuite>,
    pub compression_methods: Array<'a, CompressionMethod>,
    pub extensions: Extensions<'a, ClientExtension<'a>>,
}

impl<'a> Codec<'a> for ClientHelloPayload<'a> {
    fn encode(&self, enc: &mut Encoder<'a>) {
        self.client_version.encode(enc);
        self.random.encode(enc);
        self.session_id.encode(enc);
        self.cipher_suites.encode(enc);
        self.compression_methods.encode(enc);
        self.extensions.encode(enc);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        Some(ClientHelloPayload {
            client_version: ProtocolVersion::decode(dec)?,
            random: Random::decode(dec)?,
            session_id: SessionId::decode(dec)?,
            cipher_suites: Array::decode(dec)?,
            compression_methods: Array::decode(dec)?,
            extensions: Extensions::decode(dec)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msgs::{
        enums::SignatureScheme,
        extension::{ProtocolVersions, SignatureSchemes},
    };
    use rustls::{
        internal::msgs::{
            codec::Codec as r_Codec,
            enums::Compression as r_Compression,
            handshake::{
                ClientExtension as r_ClientExtension, ClientHelloPayload as r_ClientHelloPayload,
                Random as r_Random, SessionID as r_SessionId,
            },
        },
        CipherSuite as r_CipherSuite, ProtocolVersion as r_ProtocolVersion,
        SignatureScheme as r_SignatureScheme,
    };
    use std::vec::Vec;

    mod encode {
        use super::*;

        #[test]
        fn empty_client_hello() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_random() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[9; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::from([9; 32]),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_session_id() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::new(&[97, 98, 99]),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::from([97, 98, 99].as_ref()),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_single_cipher_suite() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![r_CipherSuite::TLS13_AES_128_GCM_SHA256],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::from([CipherSuite::TlsAes128GcmSha256].as_ref()),
                    compression_methods: Array::empty(),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_multiple_cipher_suites() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![
                        r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                        r_CipherSuite::TLS13_CHACHA20_POLY1305_SHA256
                    ],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::from(
                        [
                            CipherSuite::TlsAes128GcmSha256,
                            CipherSuite::TlsChaCha20Poly1305Sha256
                        ]
                        .as_ref()
                    ),
                    compression_methods: Array::empty(),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_single_compression_method() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![r_Compression::Deflate],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::from([CompressionMethod::Deflate].as_ref()),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_multiple_compression_methods() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![r_Compression::Null, r_Compression::Deflate],
                    extensions: vec![],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::from(
                        [CompressionMethod::Null, CompressionMethod::Deflate].as_ref()
                    ),
                    extensions: Extensions::empty(),
                })
            )
        }

        #[test]
        fn client_hello_extension_empty_signature_algorithms() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SignatureAlgorithms(vec![])],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::from(
                        [ClientExtension::SignatureAlgorithms(
                            SignatureSchemes::empty()
                        )]
                        .as_ref()
                    )
                })
            )
        }

        #[test]
        fn client_hello_extension_single_signature_algorithm() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SignatureAlgorithms(vec![
                        r_SignatureScheme::RSA_PKCS1_SHA256
                    ])],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::from(
                        [ClientExtension::SignatureAlgorithms(SignatureSchemes {
                            inner: Array::from([SignatureScheme::RSA_PKCS1_SHA256].as_ref())
                        })]
                        .as_ref()
                    )
                })
            )
        }

        #[test]
        fn client_hello_extension_multiple_signature_algorithms() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SignatureAlgorithms(vec![
                        r_SignatureScheme::RSA_PKCS1_SHA256,
                        r_SignatureScheme::ECDSA_NISTP256_SHA256,
                    ])],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::from(
                        [ClientExtension::SignatureAlgorithms(SignatureSchemes {
                            inner: Array::from(
                                [
                                    SignatureScheme::RSA_PKCS1_SHA256,
                                    SignatureScheme::ECDSA_NISTP256_SHA256,
                                ]
                                .as_ref()
                            )
                        })]
                        .as_ref()
                    )
                })
            )
        }

        #[test]
        fn client_hello_extension_empty_protocol_versions() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SupportedVersions(vec![])],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::from(
                        [ClientExtension::SupportedVersions(ProtocolVersions::empty())].as_ref()
                    )
                })
            )
        }

        #[test]
        fn client_hello_extension_single_protocol_version() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SupportedVersions(vec![
                        r_ProtocolVersion::TLSv1_3
                    ])],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::from(
                        [ClientExtension::SupportedVersions(ProtocolVersions {
                            inner: Array::from([ProtocolVersion::TLSv1_3].as_ref())
                        })]
                        .as_ref()
                    )
                })
            )
        }

        #[test]
        fn client_hello_extension_multiple_protocol_versions() {
            assert_eq!(
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_3,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SupportedVersions(vec![
                        r_ProtocolVersion::TLSv1_3,
                        r_ProtocolVersion::TLSv1_2
                    ])],
                }),
                embed_bytes(ClientHelloPayload {
                    client_version: ProtocolVersion::TLSv1_3,
                    random: Random::empty(),
                    session_id: SessionId::empty(),
                    cipher_suites: Array::empty(),
                    compression_methods: Array::empty(),
                    extensions: Extensions::from(
                        [ClientExtension::SupportedVersions(ProtocolVersions {
                            inner: Array::from(
                                [ProtocolVersion::TLSv1_3, ProtocolVersion::TLSv1_2].as_ref()
                            )
                        })]
                        .as_ref()
                    )
                })
            )
        }

        // TODO: Add test for mixed extension types.
    }

    mod decode {
        use super::*;

        #[test]
        fn empty_client_hello() {
            let bytes = rustls_bytes(r_ClientHelloPayload {
                client_version: r_ProtocolVersion::TLSv1_3,
                random: r_Random::from_slice(&[0; 32]),
                session_id: r_SessionId::empty(),
                cipher_suites: vec![],
                compression_methods: vec![],
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ClientHelloPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.client_version, ProtocolVersion::TLSv1_3);
            assert_eq!(payload.random, Random::empty());
            assert!(payload.session_id.is_empty());
            assert!(payload.cipher_suites.is_empty());
            assert!(payload.compression_methods.is_empty());
            assert!(payload.extensions.is_empty());
        }
    }

    fn rustls_bytes(payload: r_ClientHelloPayload) -> Vec<u8> {
        let mut enc = vec![];
        payload.encode(&mut enc);
        enc
    }

    fn embed_bytes(payload: ClientHelloPayload) -> Vec<u8> {
        let mut enc = Encoder::new(vec![]);
        payload.encode(&mut enc);
        enc.bytes().into()
    }
}
