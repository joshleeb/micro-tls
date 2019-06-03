use core::{u16, u8};
use managed::ManagedSlice;

pub mod enums;

#[macro_use]
mod array;

mod extension;
mod handshake;
mod primitive;
mod random;
mod session;

pub trait Codec<'a>: Sized {
    fn encode(&self, _enc: &mut Encoder<'a>);

    fn decode(_dec: &mut Decoder<'a>) -> Option<Self>;
}

// TODO: Tests for CodecSized
pub trait CodecSized<'a>: Codec<'a> {
    // TODO: Replace usize with enum to remove the unimplemented catch all in the match statements.
    // How many bytes should data_size() be put into?
    const HEADER_SIZE: usize;

    // How many bytes when this is encoded?
    fn data_size(&self) -> usize;

    fn encode_len(&self, enc: &mut Encoder<'a>) {
        match Self::HEADER_SIZE {
            0 => {}
            1 => self.encode_u8(enc),
            2 => self.encode_u16(enc),
            _ => unimplemented!(),
        }
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        match Self::HEADER_SIZE {
            0 => None,
            1 => u8::decode(dec).map(usize::from),
            2 => u16::decode(dec).map(usize::from),
            _ => unimplemented!(),
        }
    }

    fn encode_u8(&self, enc: &mut Encoder<'a>) {
        let n_bytes = self.data_size();
        debug_assert!(n_bytes <= usize::from(u8::MAX));
        (n_bytes as u8).encode(enc);
    }

    fn encode_u16(&self, enc: &mut Encoder<'a>) {
        let n_bytes = self.data_size();
        debug_assert!(n_bytes <= usize::from(u16::MAX));
        (n_bytes as u16).encode(enc);
    }
}

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

    // TODO: Writer::push should return error instead of panic??
    pub fn push(&mut self, byte: &u8) {
        match self.bytes {
            ManagedSlice::Borrowed(_) => {
                if self.is_full() {
                    panic!("not enough space to push to writer");
                }
                self.bytes.as_mut()[self.len] = *byte;
            }
            ManagedSlice::Owned(ref mut buf) => {
                buf.push(*byte);
            }
        };
        self.len += 1;
    }

    // TODO: Writer::append should return error instead of panic??
    // TODO: Writer::append shouldn't need to push multiple times if using vec (maybe use managed)
    pub fn append<B: AsRef<[u8]>>(&mut self, bytes: B) {
        match self.bytes {
            ManagedSlice::Borrowed(_) => {
                if self.remaining() < bytes.as_ref().len() {
                    panic!("not enough space to push to writer");
                }
                bytes.as_ref().iter().for_each(|b| self.push(b))
            }
            ManagedSlice::Owned(_) => bytes.as_ref().iter().for_each(|b| self.push(b)),
        }
    }

    // TODO: Maybe rename to `as_bytes` to be more consistent with the decoder.
    pub fn bytes(&self) -> &[u8] {
        match self.bytes {
            ManagedSlice::Borrowed(ref bytes) => bytes,
            ManagedSlice::Owned(ref bytes) => bytes.as_slice(),
        }
    }

    pub fn is_full(&self) -> bool {
        self.remaining() == 0
    }

    pub fn remaining(&self) -> usize {
        self.bytes.as_ref().len() - self.len
    }
}

// TODO: Tests for Decoder.
pub struct Decoder<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        self.bump_offset(len)
            .map(move |prev| &self.bytes[prev..self.offset])
    }

    pub fn sub(&mut self, len: usize) -> Option<Self> {
        self.take(len).map(Self::new)
    }

    pub fn is_complete(&self) -> bool {
        self.remaining() == 0
    }

    pub fn remaining(&self) -> usize {
        self.bytes.as_ref().len() - self.offset
    }

    pub fn as_bytes(&self) -> &'a [u8] {
        &self.bytes
    }

    /// Returns offset before the bump.
    fn bump_offset(&mut self, len: usize) -> Option<usize> {
        if self.remaining() < len {
            return None;
        }

        let prev_offset = self.offset;
        self.offset += len;
        Some(prev_offset)
    }
}
