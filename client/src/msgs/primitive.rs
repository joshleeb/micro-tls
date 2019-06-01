use crate::msgs::{Codec, CodecSized, Decoder, Encoder};
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
    const HEADER_SIZE: usize = mem::size_of::<Self>();

    fn data_size(&self) -> usize {
        mem::size_of::<Self>()
    }
}

fn put_u16(v: u16, buf: &mut [u8; 2]) {
    buf[0] = (v >> 8) as u8;
    buf[1] = v as u8;
}

impl<'a> Codec<'a> for u16 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        let mut buf = [0; 2];
        put_u16(*self, &mut buf);
        enc.append(buf);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u16::data_size(&0))
            .map(|b| (u16::from(b[0]) << 8) | u16::from(b[1]))
    }
}

impl<'a> CodecSized<'a> for u16 {
    const HEADER_SIZE: usize = mem::size_of::<Self>();

    fn data_size(&self) -> usize {
        mem::size_of::<Self>()
    }
}
