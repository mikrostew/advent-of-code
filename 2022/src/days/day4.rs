use std::path::Path;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::separated_pair;
use nom::IResult;

use super::{read_file, run_parts};

run_parts!();

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

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
        (
            next_input,
            Range {
                start: d1.parse::<i32>().expect("failed to parse i32"),
                end: d2.parse::<i32>().expect("failed to parse i32"),
            },
        )
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

fn part1<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);

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
}

fn part2<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);

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
}
