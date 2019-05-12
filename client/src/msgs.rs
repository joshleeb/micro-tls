use managed::ManagedSlice;

#[macro_use]
mod macros;

pub mod enums;

mod handshake;
mod primitive;
mod random;
mod session;
mod slice;

trait Codec<'a>: Sized {
    fn encode(&self, _enc: &mut Encoder<'a>);

    fn decode(_dec: &mut Decoder<'a>) -> Option<Self>;
}

trait CodecLength<'a> {
    /// Number of bytes.
    const LENGTH: usize;

    fn encode_len(_len: usize, _enc: &mut Encoder<'a>);

    fn decode_len(_dec: &mut Decoder<'a>) -> Option<usize>;
}

struct Encoder<'a> {
    buf: ManagedSlice<'a, u8>,
    len: usize,
}

impl<'a> Encoder<'a> {
    pub fn new<T: Into<ManagedSlice<'a, u8>>>(buf: T) -> Self {
        Self {
            buf: buf.into(),
            len: 0,
        }
    }

    // TODO: Writer::push should return error instead of panic??
    pub fn push(&mut self, byte: &u8) {
        match self.buf {
            ManagedSlice::Borrowed(_) => {
                if self.is_full() {
                    panic!("not enough space to push to writer");
                }
                self.buf.as_mut()[self.len] = *byte;
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
        match self.buf {
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
        match self.buf {
            ManagedSlice::Borrowed(ref bytes) => bytes,
            ManagedSlice::Owned(ref bytes) => bytes.as_slice(),
        }
    }

    fn is_full(&self) -> bool {
        self.remaining() == 0
    }

    fn remaining(&self) -> usize {
        self.buf.as_ref().len() - self.len
    }
}

struct Decoder<'a> {
    buf: ManagedSlice<'a, u8>,
    offset: usize,
}

impl<'a> Decoder<'a> {
    pub fn new<T: Into<ManagedSlice<'a, u8>>>(buf: T) -> Self {
        Self {
            buf: buf.into(),
            offset: 0,
        }
    }

    pub fn take(&mut self, len: usize) -> Option<&[u8]> {
        if self.remaining() < len {
            return None;
        }
        let current = self.offset;
        self.offset += len;

        Some(&self.buf[current..current + len])
    }

    pub fn is_complete(&self) -> bool {
        self.remaining() == 0
    }

    fn remaining(&self) -> usize {
        self.buf.as_ref().len() - self.offset
    }
}
