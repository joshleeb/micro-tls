use managed::ManagedSlice;

#[macro_use]
mod macros;

pub mod enums;

mod handshake;
mod primitive;
mod random;
mod session;
mod slice;

pub trait Codec<'a>: Sized {
    fn encode(&self, _enc: &mut Encoder<'a>);

    fn decode(_dec: &mut Decoder<'a>) -> Option<Self>;
}

pub trait CodecLength<'a> {
    /// Number of bytes.
    const LENGTH: usize;

    fn encode_len(_len: usize, _enc: &mut Encoder<'a>);

    fn decode_len(_dec: &mut Decoder<'a>) -> Option<usize>;
}

pub struct Encoder<'a> {
    bytes: ManagedSlice<'a, u8>,
    len: usize,
}

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
