use crate::codec::Codec;
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
impl<'a, T: Eq + Codec<'a>> Eq for Item<'a, T> {}

impl<'a, T: PartialEq + Codec<'a>> PartialEq<T> for Item<'a, T> {
    fn eq(&self, other: &T) -> bool {
        self.as_ref() == other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eq_borrowed() {
        let value: (u32, u32) = (1, 1);
        assert_eq!(Item::Borrowed(&value.0), Item::Borrowed(&value.1));
    }

    #[test]
    fn ne_borrowed() {
        let value: (u32, u32) = (1, 2);
        assert_ne!(Item::Borrowed(&value.0), Item::Borrowed(&value.1));
    }

    #[test]
    fn eq_owned() {
        let value: (u32, u32) = (1, 1);
        assert_eq!(Item::Owned(value.0), Item::Owned(value.1));
    }

    #[test]
    fn ne_owned() {
        let value: (u32, u32) = (1, 2);
        assert_ne!(Item::Owned(value.0), Item::Owned(value.1));
    }

    #[test]
    fn eq_mixed() {
        let value: (u32, u32) = (1, 1);
        assert_eq!(Item::Owned(value.0), Item::Borrowed(&value.1));
    }

    #[test]
    fn ne_mixed() {
        let value: (u32, u32) = (1, 2);
        assert_ne!(Item::Owned(value.0), Item::Borrowed(&value.1));
    }
}
