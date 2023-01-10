pub trait ToNum {
    fn to_i32(&self) -> i32;
    fn to_i64(&self) -> i64;
    fn to_isize(&self) -> isize;
    fn to_u32(&self) -> u32;
    fn to_u64(&self) -> u64;
    fn to_usize(&self) -> usize;
}

macro_rules! impl_tonum {
    ($name:ident, $T:ident) => {
        fn $name(&self) -> $T {
            self.parse::<$T>()
                .unwrap_or_else(|_| panic!("cannot parse '{}' into {}!", self, stringify!($T)))
        }
    };
}
impl ToNum for str {
    impl_tonum!(to_i32, i32);
    impl_tonum!(to_i64, i64);
    impl_tonum!(to_isize, isize);
    impl_tonum!(to_u32, u32);
    impl_tonum!(to_u64, u64);
    impl_tonum!(to_usize, usize);
}

// this is only used by the gcd function at the moment
pub trait Zero {
    fn is_zero(&self) -> bool;
}

macro_rules! impl_zero {
    ($T:ident) => {
        impl Zero for $T {
            #[inline]
            fn is_zero(&self) -> bool {
                *self == 0
            }
        }
    };
}

impl_zero!(i32);
impl_zero!(i64);
impl_zero!(isize);
impl_zero!(u32);
impl_zero!(u64);
impl_zero!(usize);
