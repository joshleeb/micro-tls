use rustls::internal::{msgs::enums::SignatureAlgorithm, pemfile};
use std::{fs::File, io::BufReader, path::PathBuf};

#[derive(PartialEq, Clone, Copy)]
pub enum KeyType {
    RSA,
    ECDSA,
}

impl KeyType {
    pub fn for_suite(suite: &'static rustls::SupportedCipherSuite) -> KeyType {
        if suite.sign == SignatureAlgorithm::ECDSA {
            return KeyType::ECDSA;
        }
        KeyType::RSA
    }

    pub fn cert_chain(&self) -> Vec<rustls::Certificate> {
        File::open(self.path("end.fullchain"))
            .map(BufReader::new)
            .map_err(|_| ())
            .and_then(|ref mut buf| pemfile::certs(buf))
            .unwrap()
    }

    pub fn priv_key(&self) -> rustls::PrivateKey {
        File::open(self.path("end.key"))
            .map(BufReader::new)
            .map_err(|_| ())
            .and_then(|ref mut buf| pemfile::pkcs8_private_keys(buf))
            .unwrap()[0]
            .clone()
    }

    pub fn ca_path(&self) -> PathBuf {
        self.path("ca.cert")
    }

    fn path(&self, key_file: &str) -> PathBuf {
        println!("{}", key_file);
        let test_keys_path = PathBuf::from("../test-keys");
        match self {
            KeyType::RSA => test_keys_path.join("rsa").join(key_file),
            KeyType::ECDSA => test_keys_path.join("ecdsa").join(key_file),
        }
    }
}
