use crate::msgs::{Codec, CodecLength, Decoder, Encoder};
use core::mem;
use managed::ManagedSlice;

impl<'a, T> Codec<'a> for ManagedSlice<'a, T>
where
    T: Sized + Codec<'a> + CodecLength<'a>,
{
    fn encode(&self, enc: &mut Encoder<'a>) {
        let len = self.len() * T::LENGTH;
        T::encode_len(len, enc);
        self.iter().for_each(|x| x.encode(enc));
    }

    /// The idea here is that we have a slice of bytes
    ///
    /// ```txt
    /// [a, b, c, d, ...]
    /// ```
    ///
    /// And we want to chunk and then map them into one of the types defined in `msgs/enums.rs`.
    /// For example, if we were to decode those bytes into `ExtensionType` (which is a `u16`), then
    /// it would become
    ///
    /// ```txt
    /// [a, b, c, d, ...]
    ///  |__|  |__|  |_|
    ///   /      \     \
    /// Ext1    Ext2   ...
    /// ```
    ///
    /// I would expect that this will end up using some unsafe code.
    ///
    /// TODO: Codec::decode should be implemented for ManagedSlice
    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        let len = T::decode_len(dec)?;
        let bytes = dec.take(len)?.chunks_exact(T::LENGTH).map(|chunk| {
            // ?????
            // unsafe {
            //     mem::transmute::<&[u8], T>(chunk)
            // }
        });

        None
    }
}
