use crate::msgs::{Codec, Decoder};
use core::{marker::PhantomData, slice};

pub enum ArrayIter<'a, T: Codec<'a>> {
    Typed(TypedArrayIter<'a, T>),
    Bytes(BytesArrayIter<'a, T>),
}

impl<'a, T: Copy + Codec<'a>> Iterator for ArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayIter::Typed(ref mut t) => t.next(),
            ArrayIter::Bytes(ref mut b) => b.next(),
        }
    }
}

impl<'a, T: Codec<'a>> From<slice::Iter<'a, T>> for ArrayIter<'a, T> {
    fn from(it: slice::Iter<'a, T>) -> Self {
        ArrayIter::Typed(TypedArrayIter { it })
    }
}

impl<'a, T: Codec<'a>> From<Decoder<'a>> for ArrayIter<'a, T> {
    fn from(dec: Decoder<'a>) -> Self {
        ArrayIter::Bytes(BytesArrayIter {
            dec,
            phantom: PhantomData,
        })
    }
}

struct TypedArrayIter<'a, T: Codec<'a>> {
    it: slice::Iter<'a, T>,
}

impl<'a, T: Copy + Codec<'a>> Iterator for TypedArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next().copied()
    }
}

struct BytesArrayIter<'a, T: Codec<'a>> {
    dec: Decoder<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T: Codec<'a>> Iterator for BytesArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::decode(&mut self.dec)
    }
}
