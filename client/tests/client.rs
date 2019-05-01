use rustls::{
    ClientConfig, ClientSession, NoClientSessionStorage, ProtocolVersion, SupportedCipherSuite,
};
use std::{fs::File, io::BufReader, path::PathBuf, sync::Arc};
use webpki::DNSNameRef;

pub fn config(
    version: ProtocolVersion,
    suite: &'static SupportedCipherSuite,
    ca_path: PathBuf,
) -> ClientConfig {
    let mut cfg = ClientConfig::new();
    let mut rootbuf = File::open(ca_path).map(BufReader::new).unwrap();

    cfg.root_store.add_pem_file(&mut rootbuf).unwrap();
    cfg.ciphersuites.clear();
    cfg.ciphersuites.push(suite);
    cfg.versions.clear();
    cfg.versions.push(version);
    cfg.set_persistence(Arc::new(NoClientSessionStorage {}));

    cfg
}

pub fn session(config: &Arc<ClientConfig>) -> ClientSession {
    let dns_name = DNSNameRef::try_from_ascii_str("localhost").unwrap();
    ClientSession::new(config, dns_name)
}
