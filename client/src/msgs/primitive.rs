use crate::msgs::{Codec, CodecLength, Decoder, Encoder};
use core::{u16, u8};

impl<'a> Codec<'a> for u8 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.push(self);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u8::LENGTH).map(|b| b[0])
    }
}

impl<'a> CodecLength<'a> for u8 {
    // TODO: CodeLength::LENGTH for u8 should look into using mem::size_of
    const LENGTH: usize = 1;

    fn encode_len(len: usize, enc: &mut Encoder<'a>) {
        debug_assert!(len <= usize::from(u8::MAX));
        (len as u8).encode(enc);
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        u8::decode(dec).map(usize::from)
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
        dec.take(u16::LENGTH)
            .map(|b| (u16::from(b[0]) << 8) | u16::from(b[1]))
    }
}

impl<'a> CodecLength<'a> for u16 {
    // TODO: CodeLength::LENGTH for u16 should look into using mem::size_of
    const LENGTH: usize = 2;

    fn encode_len(len: usize, enc: &mut Encoder<'a>) {
        debug_assert!(len <= usize::from(u16::MAX));
        (len as u16).encode(enc);
    }

    fn decode_len(dec: &mut Decoder<'a>) -> Option<usize> {
        u16::decode(dec).map(usize::from)
    }
}
