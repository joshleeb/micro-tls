use crate::error::{Error as TlsError, Result as TlsResult};
use managed::ManagedSlice;

pub struct Encoder<'a> {
    bytes: ManagedSlice<'a, u8>,
    len: usize,
}

// TODO: Tests for Encoder.
impl<'a> Encoder<'a> {
    pub fn new<T: Into<ManagedSlice<'a, u8>>>(buf: T) -> Self {
        Self {
            bytes: buf.into(),
            len: 0,
        }
    }

    pub fn push(&mut self, byte: u8) -> TlsResult<()> {
        match self.bytes {
            ManagedSlice::Borrowed(_) => {
                if self.is_full() {
                    return Err(TlsError::InternalError(
                        "not enough space to push to encoder",
                    ));
                }
                self.bytes.as_mut()[self.len] = byte;
            }
            ManagedSlice::Owned(ref mut buf) => {
                buf.push(byte);
            }
        };
        self.len += 1;
        Ok(())
    }

    // TODO: Writer::append shouldn't need to push multiple times if using vec (maybe use managed)
    pub fn append<B: AsRef<[u8]>>(&mut self, bytes: B) -> TlsResult<()> {
        let bytes_iter = match self.bytes {
            ManagedSlice::Borrowed(_) => {
                if self.remaining() < bytes.as_ref().len() {
                    return Err(TlsError::InternalError(
                        "not enough space to push to encoder",
                    ));
                }
                Ok(bytes.as_ref().iter())
            }
            ManagedSlice::Owned(_) => Ok(bytes.as_ref().iter()),
        };
        bytes_iter.and_then(|mut it| it.try_for_each(|b| self.push(*b)))
    }

    // TODO: Encoder::remaining might make sense to be different for Vec instead of [u8]
    pub fn remaining(&self) -> usize {
        self.bytes.as_ref().len() - self.len
    }

    pub fn is_full(&self) -> bool {
        self.remaining() == 0
    }

    // TODO: Maybe rename to `as_bytes` to be more consistent with the decoder.
    pub fn bytes(&self) -> &[u8] {
        match self.bytes {
            ManagedSlice::Borrowed(ref bytes) => bytes,
            ManagedSlice::Owned(ref bytes) => bytes.as_slice(),
        }
    }
}
