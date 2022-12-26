use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::separated_pair;
use nom::IResult;

use super::expect_usize;
use super::simple_struct;
use crate::cli::Params;

simple_struct!(Range; start: usize, end: usize);

impl Range {
    fn contains(&self, other: &Range) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Range) -> bool {
        (other.start >= self.start && other.start <= self.end)
            || (other.end >= self.start && other.end <= self.end)
    }
}

// ranges like "4-7", or "53-90"
// (assumption: the input is always <small>-<bigger>)
fn range(input: &str) -> IResult<&str, Range> {
    separated_pair(digit1, tag("-"), digit1)(input).map(|(next_input, (d1, d2))| {
        (next_input, Range::new(expect_usize!(d1), expect_usize!(d2)))
    })
}

// pair of ranges, like "4-5,9-12"
fn range_pair(input: &str) -> IResult<&str, (Range, Range)> {
    separated_pair(range, tag(","), range)(input)
}

fn range_is_contained(r1: &Range, r2: &Range) -> bool {
    r1.contains(r2) || r2.contains(r1)
}

fn ranges_overlap(r1: &Range, r2: &Range) -> bool {
    r1.overlaps(r2) || r2.overlaps(r1)
}

pub fn part1(file_contents: String, _p: Option<Params>) -> String {
    let contained: Vec<(Range, Range)> = file_contents
        .lines()
        .map(|line| {
            let (leftover, ranges) = range_pair(line).expect("failed to parse line");
            assert_eq!(leftover, "");
            //println!("ranges: {:?}", ranges);
            ranges
        })
        .filter(|(r1, r2)| range_is_contained(r1, r2))
        .collect();

    //println!("contained: {:?}", contained);
    println!("total: {}", contained.len());
    format!("{}", contained.len())
}

pub fn part2(file_contents: String, _p: Option<Params>) -> String {
    let overlap: Vec<(Range, Range)> = file_contents
        .lines()
        .map(|line| {
            let (leftover, ranges) = range_pair(line).expect("failed to parse line");
            assert_eq!(leftover, "");
            //println!("ranges: {:?}", ranges);
            ranges
        })
        .filter(|(r1, r2)| ranges_overlap(r1, r2))
        .collect();

    //println!("overlap: {:?}", overlap);
    println!("total: {}", overlap.len());
    format!("{}", overlap.len())
}

#[cfg(test)]
mod tests {
    use crate::days::test::aoc_test;

    aoc_test!(part1_example: "day4", part1, "example", 2);
    aoc_test!(part1_input: "day4", part1, "input", 538);

    aoc_test!(part2_example: "day4", part2, "example", 4);
    aoc_test!(part2_input: "day4", part2, "input", 792);
}
