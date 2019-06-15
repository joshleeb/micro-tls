use crate::codec::{decoder::Decoder, encoder::Encoder, Codec, CodecSized, HeaderSize};
use core::{mem, u16, u8};

impl<'a> Codec<'a> for u8 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.push(self);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u8::data_size(&0)).map(|b| b[0])
    }
}

impl<'a> CodecSized<'a> for u8 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U8;

    fn data_size(&self) -> usize {
        mem::size_of::<Self>()
    }
}

impl<'a> Codec<'a> for u16 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        let buf = [(*self >> 8) as u8, *self as u8];
        enc.append(buf);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u16::data_size(&0))
            .map(|b| (u16::from(b[0]) << 8) | u16::from(b[1]))
    }
}

impl<'a> CodecSized<'a> for u16 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U16;

    fn data_size(&self) -> usize {
        mem::size_of::<Self>()
    }
}
