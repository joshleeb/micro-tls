use rustls::{
    Certificate, NoClientAuth, NoServerSessionStorage, PrivateKey, ProtocolVersion, ServerConfig,
    ServerSession,
};
use std::sync::Arc;

pub fn config(
    version: ProtocolVersion,
    cert_chain: Vec<Certificate>,
    priv_key: PrivateKey,
) -> ServerConfig {
    let mut cfg = ServerConfig::new(NoClientAuth::new());

    cfg.set_single_cert(cert_chain, priv_key)
        .expect("bad certs/private key?");
    cfg.set_persistence(Arc::new(NoServerSessionStorage {}));
    cfg.versions.clear();
    cfg.versions.push(version);

    cfg
}

pub fn session(config: &Arc<ServerConfig>) -> ServerSession {
    ServerSession::new(config)
}
