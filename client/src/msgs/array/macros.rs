macro_rules! arr {
    ($($x:expr),*) => (
        $crate::msgs::array::Array::from([$($x),*].as_ref())
    );
    ($($x:expr,)*) => (arr![$($x),*])
}
