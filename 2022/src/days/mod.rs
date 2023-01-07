use std::ops::Rem;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::sequence::tuple;
use nom::IResult;
use seq_macro::seq;

seq!(N in 1..=25 {
    pub mod day~N;
});

// TODO: split this stuff into a utils crate

// because I do this all the time
macro_rules! simple_struct {
    // not default deriving Copy because some structs use String
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
    // but can add things to the derive if needed
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
pub(crate) use simple_struct;

// meta-macro for making these macros
macro_rules! num_parsing_macro {
    ($name:ident, $type:ident) => {
        macro_rules! $name {
            ($e:ident) => {
                $e.parse::<$type>()
                    .unwrap_or_else(|_| panic!("cannot parse {} into {}!", $e, stringify!($type)))
            };
        }
        pub(crate) use $name;
    };
}
num_parsing_macro!(expect_usize, usize);
num_parsing_macro!(expect_isize, isize);
num_parsing_macro!(expect_i32, i32);

// TODO: I should also macro-ize these fns

// parse unsigned int into usize
pub(crate) fn parse_usize(input: &str) -> IResult<&str, usize> {
    map(digit1, |n: &str| {
        n.parse::<usize>().expect("failed to parse usize!")
    })(input)
}

// parse signed ints (e.g. 4, -67, 234) into isize
pub(crate) fn parse_isize(input: &str) -> IResult<&str, isize> {
    map(recognize(tuple((opt(tag("-")), digit1))), |n: &str| {
        n.parse::<isize>().expect("failed to parse isize!")
    })(input)
}

// parse signed ints (e.g. 4, -67, 234) into i32
pub(crate) fn parse_i32(input: &str) -> IResult<&str, i32> {
    map(recognize(tuple((opt(tag("-")), digit1))), |n: &str| {
        n.parse::<i32>().expect("failed to parse i32!")
    })(input)
}

pub(crate) trait Zero {
    fn is_zero(&self) -> bool;
}

// TODO: macro-ize this to do for more int types
impl Zero for usize {
    fn is_zero(&self) -> bool {
        *self == 0
    }
}

// find GCD of 2 numbers
// (Euclid's algorithm)
pub(crate) fn gcd<T>(x: T, y: T) -> T
where
    T: Copy + Rem<Output = T> + Zero,
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
