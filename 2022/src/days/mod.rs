use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::multi::many_m_n;
use nom::sequence::tuple;
use nom::IResult;

pub mod day1;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day2;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod day9;

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

// parse signed ints (e.g. 4, -67, 234) into i32
pub(crate) fn parse_i32(input: &str) -> IResult<&str, i32> {
    tuple((many_m_n(0, 1, tag("-")), digit1))(input).map(|(next_input, (sign, dig))| {
        (
            next_input,
            format!("{}{}", sign.join(""), dig)
                .parse::<i32>()
                .expect("failed to parse i32!"),
        )
    })
}

pub(crate) use expect_i32;
pub(crate) use expect_usize;

// test helpers

#[cfg(test)]
fn read_input_file(path: &str) -> String {
    let file_contents = std::fs::read_to_string(path).expect("failed to read file");
    file_contents
}
