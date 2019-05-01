use keytype::KeyType;
use rustls::{ProtocolVersion, Session, SupportedCipherSuite};
use std::sync::Arc;

mod client;
mod keytype;
mod server;
mod suite;

#[test]
fn handshake_tls13_chacha20_poly1305_sha256() {
    test_handshake(&suite::TLS13_CHACHA20_POLY1305_SHA256)
}

#[test]
fn handshake_tls13_aes_256_gcm_sha384() {
    test_handshake(&suite::TLS13_AES_256_GCM_SHA384)
}

#[test]
fn handshake_tls13_aes_128_gcm_sha256() {
    test_handshake(&suite::TLS13_AES_128_GCM_SHA256);
}

fn test_handshake(suite: &'static SupportedCipherSuite) {
    let kt = KeyType::for_suite(suite);
    let client_config = client::config(ProtocolVersion::TLSv1_3, suite, kt.ca_path());
    let server_config = server::config(ProtocolVersion::TLSv1_3, kt.cert_chain(), kt.priv_key());

    let mut client = client::session(&Arc::new(client_config));
    let mut server = server::session(&Arc::new(server_config));

    while server.is_handshaking() || client.is_handshaking() {
        transfer(&mut client, &mut server);
        server.process_new_packets().unwrap();

        transfer(&mut server, &mut client);
        client.process_new_packets().unwrap();
    }
}

fn transfer(left: &mut dyn Session, right: &mut dyn Session) {
    let mut buf = [0u8; 262144];

    while left.wants_write() {
        let written = left.write_tls(&mut buf.as_mut()).unwrap();
        if written == 0 {
            return;
        }

        let mut offset = 0;
        loop {
            offset += right.read_tls(&mut buf[offset..written].as_ref()).unwrap();
            if written == offset {
                break;
            }
        }
    }
}
