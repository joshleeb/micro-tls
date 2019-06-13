macro_rules! arr {
    ($($x:expr),*) => (
        $crate::array::Array::from([$($x),*].as_ref())
    );
    ($($x:expr,)*) => (arr![$($x),*])
}
