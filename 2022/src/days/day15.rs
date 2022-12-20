use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::parse_isize;
use super::simple_struct;

simple_struct!(Point; x: isize, y: isize);

impl Point {
    // Manhattan Distance between 2 points
    fn manhattan_dist(&self, other: &Self) -> isize {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    beacon: Point,
    exclusion_dist: isize,
}

impl Sensor {
    fn new(location: Point, beacon: Point) -> Self {
        // this dist is same as dist to beacon
        // (no other beacons can exist at that distance)
        let exclusion_dist = location.manhattan_dist(&beacon);
        Sensor {
            location,
            beacon,
            exclusion_dist,
        }
    }

    fn exclusion_for_row(&self, y: isize) -> Option<Exclusion> {
        let dist_from_row = (self.location.y - y).abs();
        // if it's too far away, there is no exclusion
        if dist_from_row > self.exclusion_dist {
            return None;
        }
        // otherwise, figure out where things are excluded
        let midpoint = self.location.x;
        let dist_on_row = self.exclusion_dist - dist_from_row;
        Some(Exclusion::new(
            midpoint - dist_on_row,
            midpoint + dist_on_row,
        ))
    }
}

// exclusion for a single row
simple_struct!(Exclusion; start: isize, end: isize);

impl Exclusion {
    // try to combine with another, possibly overlapping, exclusion
    fn combine_with(&self, other: &Self) -> Option<Exclusion> {
        if self.start < other.start {
            // no overlap
            // (if they are contiguous, that's ok to combine)
            if self.end < other.start - 1 {
                return None;
            } else {
                if self.end < other.end {
                    return Some(Exclusion {
                        start: self.start,
                        end: other.end,
                    });
                } else {
                    // self.end >= other.end
                    return Some(Exclusion {
                        start: self.start,
                        end: self.end,
                    });
                }
            }
        } else if self.start == other.start {
            if self.end < other.end {
                return Some(Exclusion {
                    start: self.start,
                    end: other.end,
                });
            } else {
                // self.end >= other.end
                return Some(Exclusion {
                    start: self.start,
                    end: self.end,
                });
            }
        } else {
            // self.start > other.start
            // no overlap
            if self.start > other.end {
                return None;
            } else {
                // self.start <= other.end
                if self.end < other.end {
                    return Some(Exclusion {
                        start: other.start,
                        end: other.end,
                    });
                } else {
                    // self.end >= other.end
                    return Some(Exclusion {
                        start: other.start,
                        end: self.end,
                    });
                }
            }
        }
    }
}

fn parse_sensors(input: &str) -> Vec<Sensor> {
    let (leftover, s) = sensors(input).expect("Could not parse sensors");
    assert_eq!(leftover, "");
    s
}

fn sensors(input: &str) -> IResult<&str, Vec<Sensor>> {
    terminated(separated_list1(newline, sensor), newline)(input)
}

fn sensor(input: &str) -> IResult<&str, Sensor> {
    map(tuple((location, beacon)), |(l, b)| Sensor::new(l, b))(input)
}

fn location(input: &str) -> IResult<&str, Point> {
    map(
        preceded(
            tag("Sensor at x="),
            separated_pair(parse_isize, tag(", y="), parse_isize),
        ),
        |(x, y)| Point::new(x, y),
    )(input)
}

fn beacon(input: &str) -> IResult<&str, Point> {
    map(
        preceded(
            tag(": closest beacon is at x="),
            separated_pair(parse_isize, tag(", y="), parse_isize),
        ),
        |(x, y)| Point::new(x, y),
    )(input)
}

fn combine_exclusions(sensors: &Vec<Sensor>, row: isize) -> Vec<Exclusion> {
    // figure out exclusions
    let mut excls: Vec<Exclusion> = sensors
        .iter()
        .filter_map(|s| s.exclusion_for_row(row))
        .collect();

    // sort by starting point (to make combining easier)
    excls.sort_by_key(|e| e.start);

    let mut combined_exclusions: Vec<Exclusion> = Vec::new();
    let mut combiner = excls[0].clone();
    for i in 1..excls.len() {
        if let Some(combined) = combiner.combine_with(&excls[i]) {
            combiner = combined;
            //println!("combined");
        } else {
            // couldn't combine, save and try with the next one
            combined_exclusions.push(combiner);
            combiner = excls[i].clone();
            //println!("not combined");
        }
    }
    combined_exclusions.push(combiner);
    combined_exclusions
}

fn count_exclusions(excls: &Vec<Exclusion>, beacons_x: &Vec<isize>) -> isize {
    // nested for loop, maybe slow? (but the lengths are small, so ¯\_(ツ)_/¯ )
    let mut exclusions = 0;
    for e in excls.iter() {
        let mut count = e.end - e.start + 1;
        for x in beacons_x.iter() {
            if e.start <= *x && *x <= e.end {
                count -= 1;
            }
        }
        exclusions += count;
    }
    exclusions
}

fn exclusions_in_row(y: isize, sensors: &Vec<Sensor>) -> isize {
    // combine exclusions
    let combined = combine_exclusions(&sensors, y);
    println!("exclusions: {:?}", combined);

    // find x-values of any beacons on that row (intermediate set to handle uniques)
    let beacons_x: Vec<isize> = sensors
        .iter()
        .filter_map(|s| {
            if s.beacon.y == y {
                Some(s.beacon.x)
            } else {
                None
            }
        })
        .collect::<HashSet<isize>>()
        .into_iter()
        .collect();
    println!("beacons: {:?}", beacons_x);

    // count, accounting for any beacons existing on the row
    count_exclusions(&combined, &beacons_x)
}

fn find_gap_in_range(exclusions: &Vec<Exclusion>, min: isize, max: isize) -> Option<isize> {
    let mut gap_start: Option<isize> = None;
    let mut gap_end: Option<isize> = None;

    for e in exclusions.iter() {
        if e.start <= min {
            if e.end >= max {
                break;
            } else {
                // e.end < max
                if let Some(_) = gap_start {
                    panic!("Already found a gap start!");
                } else {
                    gap_start = Some(e.end + 1);
                }
            }
        } else {
            // e.start > min
            if let Some(_) = gap_end {
                panic!("Already found a gap end!");
            } else {
                gap_end = Some(e.start - 1);
            }
        }
    }

    // figure out if the gap makes sense
    if let Some(start) = gap_start {
        // is there also an end?
        if let Some(end) = gap_end {
            if end == start {
                Some(start)
            } else {
                panic!("Gap is too large!");
            }
        } else {
            if start == max - 1 {
                Some(max)
            } else {
                panic!("Gap at end is >1");
            }
        }
    } else {
        // if we only found an end, then it's at the beginning
        if let Some(end) = gap_end {
            if end - min == 1 {
                Some(min)
            } else {
                panic!("Gap at beginning is >1");
            }
        } else {
            None
        }
    }
}

fn find_beacon(
    sensors: &Vec<Sensor>,
    x_min: isize,
    x_max: isize,
    y_min: isize,
    y_max: isize,
) -> Point {
    let beacons: Vec<Point> = (y_min..=y_max)
        .filter_map(|y| {
            let exclusions = combine_exclusions(&sensors, y);
            println!("{}: {:?}", y, exclusions);

            if let Some(x) = find_gap_in_range(&exclusions, x_min, x_max) {
                Some(Point::new(x, y))
            } else {
                None
            }
        })
        .collect();
    if beacons.len() == 1 {
        beacons[0].clone()
    } else {
        panic!("Found multiple beacons!");
    }
}

// TODO: figure out how to accommodate another param in this function (and the CLI)
pub fn part1(file_contents: String) -> String {
    //println!("{}", file_contents);
    let sensors: Vec<Sensor> = parse_sensors(&file_contents);

    // for the example
    //let exclusions = exclusions_in_row(10, &sensors);
    let exclusions = exclusions_in_row(2_000_000, &sensors);

    format!("{}", exclusions)
}

// TODO: figure out how to accommodate another param in this function (and the CLI)
pub fn part2(file_contents: String) -> String {
    //println!("{}", file_contents);
    let sensors: Vec<Sensor> = parse_sensors(&file_contents);

    // for the example
    let beacon_pt = find_beacon(&sensors, 0, 20, 0, 20);
    // for the real input
    //let beacon_pt = find_beacon(&sensors, 0, 4_000_000, 0, 4_000_000);

    format!("{}", beacon_pt.x * 4_000_000 + beacon_pt.y)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    // TODO: this doesn't work now :(
    // #[test]
    // fn part1_example() {
    //     let input = read_input_file("inputs/day15-example.txt");
    //     let output = part1(input);
    //     let expected = format!("{}", 26);
    //     assert_eq!(output, expected);
    // }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day15-input.txt");
        assert_eq!(part1(input), "5525847".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day15-example.txt");
        assert_eq!(part2(input), "56000011".to_string());
    }

    // this takes a long time to run
    // TODO: and also doesn't work - needs an extra param :(
    // #[test]
    // fn part2_input() {
    //     let input = read_input_file("inputs/day15-input.txt");
    //     assert_eq!(part2(input), "13340867187704".to_string());
    // }
}
