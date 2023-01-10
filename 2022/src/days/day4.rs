use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::separated_pair;
use nom::IResult;

use run_aoc::runner_fn;
use utils::{nom_usize, simple_struct};

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
    map(
        separated_pair(nom_usize, tag("-"), nom_usize),
        |(d1, d2)| Range::new(d1, d2),
    )(input)
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

#[runner_fn]
fn part1(file_contents: String) -> usize {
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
    contained.len()
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
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
    overlap.len()
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day4, part1, example, 2);
    test_fn!(day4, part1, input, 538);

    test_fn!(day4, part2, example, 4);
    test_fn!(day4, part2, input, 792);
}
