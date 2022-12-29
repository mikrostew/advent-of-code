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

// TODO: split this stuff into a utils crate or something?

// because I do this all the time
// (not deriving Copy because some structs use String)
macro_rules! simple_struct {
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
}

// who needs error handling, this ain't production code
macro_rules! expect_usize {
    ($e:ident) => {
        $e.parse::<usize>().expect("failed to parse usize!")
    }
}
macro_rules! expect_i32 {
    ($e:ident) => {
        $e.parse::<i32>().expect("failed to parse i32!")
    };
}
pub(crate) use expect_i32;
pub(crate) use expect_usize;
pub(crate) use simple_struct;

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
