use crate::{
    array::item::Item,
    codec::{decoder::Decoder, Codec},
};
use core::{marker::PhantomData, slice};

pub enum ArrayIter<'a, T: Codec<'a>> {
    Typed(TypedArrayIter<'a, T>),
    Bytes(BytesArrayIter<'a, T>),
}

impl<'a, T: Codec<'a>> Iterator for ArrayIter<'a, T> {
    type Item = Item<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ArrayIter::Typed(ref mut t) => t.next().map(Item::Borrowed),
            ArrayIter::Bytes(ref mut b) => b.next().map(Item::Owned),
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

pub struct TypedArrayIter<'a, T: Codec<'a>> {
    it: slice::Iter<'a, T>,
}

impl<'a, T: Codec<'a>> Iterator for TypedArrayIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.it.next()
    }
}

pub struct BytesArrayIter<'a, T: Codec<'a>> {
    dec: Decoder<'a>,
    phantom: PhantomData<T>,
}

impl<'a, T: Codec<'a>> Iterator for BytesArrayIter<'a, T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::decode(&mut self.dec)
    }
}
