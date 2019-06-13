#![no_std]

#[cfg(test)]
#[macro_use]
extern crate std;

use mtls_message::{enums::ProtocolVersion, error::Result as TlsResult};

pub struct Config {
    version: ProtocolVersion,
}

impl Config {
    pub fn new(version: ProtocolVersion) -> Self {
        Config { version }
    }
}

pub struct Handshake {
    config: Config,
}

impl Handshake {
    pub fn new(config: Config) -> Self {
        Handshake { config }
    }

    pub fn start(&mut self, _buf: &mut [u8]) -> TlsResult<usize> {
        Ok(0)
    }

    pub fn finish(&mut self, _buf: &mut [u8]) -> TlsResult<(usize, Session)> {
        Ok((0, Session::new()))
    }

    pub fn is_handshaking(&self) -> bool {
        false
    }

    pub fn read(&mut self, _buf: &[u8]) -> TlsResult<()> {
        Ok(())
    }
}

pub struct Session {}

impl Session {
    fn new() -> Self {
        Session {}
    }

    fn encrypt(&mut self, _buf: &mut [u8]) -> TlsResult<usize> {
        Ok(0)
    }

    fn decrypt(&mut self, _buf: &mut [u8]) -> TlsResult<usize> {
        Ok(0)
    }
}
