use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::sequence::separated_pair;
use nom::IResult;

use super::expect_usize;
use super::simple_struct;

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

pub fn part1(file_contents: String) -> String {
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

pub fn part2(file_contents: String) -> String {
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
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day4-example.txt");
        assert_eq!(part1(input), "2".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day4-input.txt");
        assert_eq!(part1(input), "538".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day4-example.txt");
        assert_eq!(part2(input), "4".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day4-input.txt");
        assert_eq!(part2(input), "792".to_string());
    }
}
