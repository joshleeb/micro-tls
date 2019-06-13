// TODO: Tests for Decoder.
pub struct Decoder<'a> {
    bytes: &'a [u8],
    offset: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, offset: 0 }
    }

    pub fn take(&mut self, len: usize) -> Option<&'a [u8]> {
        self.bump_offset(len)
            .map(move |prev| &self.bytes[prev..self.offset])
    }

    pub fn sub(&mut self, len: usize) -> Option<Self> {
        self.take(len).map(Self::new)
    }

    pub fn remaining(&self) -> usize {
        self.bytes.as_ref().len() - self.offset
    }

    pub fn is_complete(&self) -> bool {
        self.remaining() == 0
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
