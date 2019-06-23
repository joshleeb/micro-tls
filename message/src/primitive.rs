use crate::codec::{decoder::Decoder, encoder::Encoder, Codec, CodecSized, HeaderSize};
use core::{u16, u32, u8};

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
        1
    }
}

impl<'a> Codec<'a> for u16 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.append([(*self >> 8) as u8, *self as u8]);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u16::data_size(&0))
            .map(|b| (u16::from(b[0]) << 8) | u16::from(b[1]))
    }
}

impl<'a> CodecSized<'a> for u16 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U16;

    fn data_size(&self) -> usize {
        2
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
pub struct u24(u32);

impl u24 {
    pub fn as_u32(self) -> u32 {
        self.0
    }
}

impl<'a> Codec<'a> for u24 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.append([(self.0 >> 16) as u8, (self.0 >> 8) as u8, self.0 as u8]);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u32::data_size(&0))
            .map(|b| Self((u32::from(b[0]) << 16) | (u32::from(b[1]) << 8) | u32::from(b[2])))
    }
}

impl<'a> CodecSized<'a> for u24 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U24;

    fn data_size(&self) -> usize {
        3
    }
}

impl From<u32> for u24 {
    fn from(data: u32) -> Self {
        Self(data)
    }
}

impl<'a> Codec<'a> for u32 {
    fn encode(&self, enc: &mut Encoder<'a>) {
        enc.append([
            (*self >> 24) as u8,
            (*self >> 16) as u8,
            (*self >> 8) as u8,
            *self as u8,
        ]);
    }

    fn decode(dec: &mut Decoder<'a>) -> Option<Self> {
        dec.take(u32::data_size(&0)).map(|b| {
            (u32::from(b[0]) << 24)
                | (u32::from(b[1]) << 16)
                | (u32::from(b[2]) << 8)
                | u32::from(b[3])
        })
    }
}

impl<'a> CodecSized<'a> for u32 {
    const HEADER_SIZE: HeaderSize = HeaderSize::U32;

    fn data_size(&self) -> usize {
        4
    }
}
