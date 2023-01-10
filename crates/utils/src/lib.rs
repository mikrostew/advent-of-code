use std::ops::Rem;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::IResult;

pub mod traits;

// find GCD of 2 numbers
// (Euclid's algorithm)
pub fn gcd<T>(x: T, y: T) -> T
where
    T: Copy + Rem<Output = T> + traits::Zero,
{
    let mut a = x;
    let mut b = y;
    while !b.is_zero() {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

// simple struct with fields and new() method
#[macro_export]
macro_rules! simple_struct {
    // example: simple_struct!(Point; x: usize, y: usize)
    // (not default deriving Copy because some structs use String)
    ($s:ident; $($v:ident: $t:ty),+) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq)]
        struct $s {
            $($v: $t),+
        }

        impl $s {
            fn new($($v: $t),+) -> Self {
                $s {
                    $($v),+
                }
            }
        }
    };
    // can add things to the derive if needed
    // example: simple_struct!([Copy] Pos; row: usize, col: usize)
    ([$($d:ident),+] $s:ident; $($v:ident: $t:ty),+) => {
        #[derive(Clone, Debug, Hash, Eq, PartialEq, $($d),+)]
        struct $s {
            $($v: $t),+
        }

        impl $s {
            fn new($($v: $t),+) -> Self {
                $s {
                    $($v),+
                }
            }
        }
    };
}

// number parsing for nom

macro_rules! nom_unum_parser {
    ($name:ident, $T:ident) => {
        pub fn $name(input: &str) -> IResult<&str, $T> {
            map(digit1, |n: &str| {
                n.parse::<$T>()
                    .unwrap_or_else(|_| panic!("failed to parse '{}' into {}!", n, stringify!($T)))
            })(input)
        }
    };
}
macro_rules! nom_inum_parser {
    ($name:ident, $T:ident) => {
        pub fn $name(input: &str) -> IResult<&str, $T> {
            map(recognize(tuple((opt(tag("-")), digit1))), |n: &str| {
                n.parse::<$T>()
                    .unwrap_or_else(|_| panic!("failed to parse '{}' into {}!", n, stringify!($T)))
            })(input)
        }
    };
}

nom_unum_parser!(nom_u32, u32);
nom_unum_parser!(nom_u64, u64);
nom_unum_parser!(nom_usize, usize);

nom_inum_parser!(nom_i32, i32);
nom_inum_parser!(nom_i64, i64);
nom_inum_parser!(nom_isize, isize);
