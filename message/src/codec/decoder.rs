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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_bytes() {
        let bytes = [1, 2, 3];
        let mut dec = Decoder::new(&bytes);
        let taken_bytes = dec.take(2).unwrap();

        assert_eq!(taken_bytes, [1, 2]);
        assert_eq!(dec.offset, 2);
    }

    #[test]
    fn taken_bytes_invalid() {
        let bytes = [1, 2, 3];
        let mut dec = Decoder::new(&bytes);
        let taken_bytes = dec.take(4);

        assert!(taken_bytes.is_none());
    }

    #[test]
    fn sub_decoder() {
        let bytes = [1, 2, 3];
        let mut dec = Decoder::new(&bytes);
        let sub_dec = dec.sub(2).unwrap();

        assert_eq!(sub_dec.offset, 0);
        assert_eq!(sub_dec.bytes, &[1, 2]);
        assert_eq!(dec.offset, 2);
    }

    #[test]
    fn sub_decoder_invalid() {
        let bytes = [1, 2, 3];
        let mut dec = Decoder::new(&bytes);
        let sub_dec = dec.sub(4);

        assert!(sub_dec.is_none());
        assert_eq!(dec.offset, 0);
    }

    #[test]
    fn bytes_reamining() {
        let bytes = [1, 2, 3];
        let mut dec = Decoder::new(&bytes);
        dec.take(2).unwrap();

        assert_eq!(dec.remaining(), 1);
    }

    #[test]
    fn complete() {
        let bytes = [1, 2, 3];
        let mut dec = Decoder::new(&bytes);

        dec.take(2).unwrap();
        assert!(!dec.is_complete());

        dec.take(1).unwrap();
        assert!(dec.is_complete());
    }
}
