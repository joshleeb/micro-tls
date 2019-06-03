use crate::msgs::{array::iter::ArrayIter, Codec, CodecSized, Decoder, Encoder};
use core::borrow::Borrow;

#[derive(Debug, Clone)]
pub enum Item<'a, T: Codec<'a>> {
    Borrowed(&'a T),
    Owned(T),
}

impl<'a, T: Codec<'a>> AsRef<T> for Item<'a, T> {
    fn as_ref(&self) -> &T {
        self.borrow()
    }
}

impl<'a, T: Codec<'a>> Borrow<T> for Item<'a, T> {
    fn borrow(&self) -> &T {
        match self {
            Item::Borrowed(ref_t) => ref_t,
            Item::Owned(t) => &t,
        }
    }
}

impl<'a, T: PartialEq + Codec<'a>> PartialEq for Item<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<'a, T: PartialEq + Codec<'a>> PartialEq<T> for Item<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other
    }
}
