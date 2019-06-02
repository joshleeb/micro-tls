use crate::msgs::{
    array::Array,
    enums::{CipherSuite, CompressionMethod, ProtocolVersion},
    extension::{client::ClientExtension, Extensions},
    random::Random,
    session::SessionId,
    Codec, Decoder, Encoder,
};

#[derive(Debug, Default)]
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
                embed_bytes(ClientHelloPayload::default()),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_random() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    random: [9; 32].into(),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[9; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_session_id() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    session_id: [97, 98, 99].into(),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::new(&[97, 98, 99]),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_single_cipher_suite() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    cipher_suites: arr![CipherSuite::TlsAes128GcmSha256],
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![r_CipherSuite::TLS13_AES_128_GCM_SHA256],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_multiple_cipher_suites() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    cipher_suites: arr![
                        CipherSuite::TlsAes128GcmSha256,
                        CipherSuite::TlsChaCha20Poly1305Sha256,
                    ],
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![
                        r_CipherSuite::TLS13_AES_128_GCM_SHA256,
                        r_CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
                    ],
                    compression_methods: vec![],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_single_compression_method() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    compression_methods: arr![CompressionMethod::Deflate],
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![r_Compression::Deflate],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_multiple_compression_methods() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    compression_methods: arr![CompressionMethod::Null, CompressionMethod::Deflate],
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![r_Compression::Null, r_Compression::Deflate],
                    extensions: vec![],
                }),
            )
        }

        #[test]
        fn client_hello_extension_empty_signature_algorithms() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![ClientExtension::SignatureAlgorithms(
                        SignatureSchemes::empty(),
                    )]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SignatureAlgorithms(vec![])],
                }),
            )
        }

        #[test]
        fn client_hello_extension_single_signature_algorithm() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![ClientExtension::from(arr![
                        SignatureScheme::RsaPkcs1Sha256,
                    ])]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SignatureAlgorithms(vec![
                        r_SignatureScheme::RSA_PKCS1_SHA256,
                    ])],
                }),
            )
        }

        #[test]
        fn client_hello_extension_multiple_signature_algorithms() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![ClientExtension::from(arr![
                        SignatureScheme::RsaPkcs1Sha256,
                        SignatureScheme::EcdsaNistp256Sha256,
                    ])]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SignatureAlgorithms(vec![
                        r_SignatureScheme::RSA_PKCS1_SHA256,
                        r_SignatureScheme::ECDSA_NISTP256_SHA256,
                    ])],
                }),
            )
        }

        #[test]
        fn client_hello_extension_empty_protocol_versions() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![ClientExtension::SupportedVersions(
                        ProtocolVersions::empty(),
                    )]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SupportedVersions(vec![])],
                }),
            )
        }

        #[test]
        fn client_hello_extension_single_protocol_version() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![ClientExtension::from(arr![
                        ProtocolVersion::TLSv1_3,
                    ])]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SupportedVersions(vec![
                        r_ProtocolVersion::TLSv1_3,
                    ])],
                }),
            )
        }

        #[test]
        fn client_hello_extension_multiple_protocol_versions() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![ClientExtension::from(arr![
                        ProtocolVersion::TLSv1_3,
                        ProtocolVersion::TLSv1_2,
                    ])]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![r_ClientExtension::SupportedVersions(vec![
                        r_ProtocolVersion::TLSv1_3,
                        r_ProtocolVersion::TLSv1_2,
                    ])],
                }),
            )
        }

        #[test]
        fn client_hello_extension_multiple_types() {
            assert_eq!(
                embed_bytes(ClientHelloPayload {
                    extensions: Extensions::from(arr![
                        ClientExtension::from(arr![
                            ProtocolVersion::TLSv1_3,
                            ProtocolVersion::TLSv1_2,
                        ]),
                        ClientExtension::from(arr![
                            SignatureScheme::RsaPkcs1Sha256,
                            SignatureScheme::EcdsaNistp256Sha256,
                        ])
                    ]),
                    ..Default::default()
                }),
                rustls_bytes(r_ClientHelloPayload {
                    client_version: r_ProtocolVersion::TLSv1_2,
                    random: r_Random::from_slice(&[0; 32]),
                    session_id: r_SessionId::empty(),
                    cipher_suites: vec![],
                    compression_methods: vec![],
                    extensions: vec![
                        r_ClientExtension::SupportedVersions(vec![
                            r_ProtocolVersion::TLSv1_3,
                            r_ProtocolVersion::TLSv1_2,
                        ]),
                        r_ClientExtension::SignatureAlgorithms(vec![
                            r_SignatureScheme::RSA_PKCS1_SHA256,
                            r_SignatureScheme::ECDSA_NISTP256_SHA256,
                        ])
                    ],
                }),
            )
        }
    }

    mod decode {
        use super::*;

        #[test]
        fn empty_client_hello() {
            let bytes = rustls_bytes(r_ClientHelloPayload {
                client_version: r_ProtocolVersion::TLSv1_2,
                random: r_Random::from_slice(&[0; 32]),
                session_id: r_SessionId::empty(),
                cipher_suites: vec![],
                compression_methods: vec![],
                extensions: vec![],
            });
            let mut dec = Decoder::new(&bytes);
            let payload = ClientHelloPayload::decode(&mut dec).unwrap();

            assert_eq!(payload.client_version, ProtocolVersion::TLSv1_2);
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
