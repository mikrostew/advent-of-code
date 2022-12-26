use std::cmp::max;
use std::cmp::Ordering;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::expect_usize;
use super::simple_struct;
use crate::cli::Params;

simple_struct!(PacketPair; left: Vec<ListOrInt>, right: Vec<ListOrInt>);
simple_struct!(Packet; parsed: Vec<ListOrInt>, orig: String);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum ListOrInt {
    List(Vec<ListOrInt>),
    Int(usize),
}

fn list(input: &str) -> IResult<&str, ListOrInt> {
    map(delimited(tag("["), list_or_int, tag("]")), |res| {
        ListOrInt::List(res)
    })(input)
}

fn int(input: &str) -> IResult<&str, ListOrInt> {
    map(digit1, |d: &str| ListOrInt::Int(expect_usize!(d)))(input)
}

fn list_or_int(input: &str) -> IResult<&str, Vec<ListOrInt>> {
    separated_list0(tag(","), alt((int, list)))(input)
}

fn packet(input: &str) -> IResult<&str, Vec<ListOrInt>> {
    delimited(tag("["), list_or_int, tag("]"))(input)
}

fn packet_line(input: &str) -> IResult<&str, Vec<ListOrInt>> {
    terminated(packet, newline)(input)
}

fn packet_pair(input: &str) -> IResult<&str, PacketPair> {
    map(tuple((packet_line, packet_line)), |(p1, p2)| {
        PacketPair::new(p1, p2)
    })(input)
}

fn parse_packets(input: &str) -> Vec<PacketPair> {
    let (leftover, pair) =
        separated_list1(newline, packet_pair)(input).expect("Could not parse packets!");
    assert_eq!(leftover, "");
    pair
}

// true if order is correct, false otherwise
fn check_order(pair: &PacketPair) -> bool {
    let left = &pair.left;
    let right = &pair.right;
    if let Some(result) = compare_vec(left, right) {
        return result;
    }
    // if everything compared the same, then it's the wrong order I guess?
    println!("all was the same, so wrong?");
    false
}

fn compare_vec(left: &Vec<ListOrInt>, right: &Vec<ListOrInt>) -> Option<bool> {
    let max_len = max(left.len(), right.len());
    for i in 0..max_len {
        // if left runs out first, that's the right order
        if left.len() == i {
            return Some(true);
        }
        // if right runs out first that's wrong
        if right.len() == i {
            return Some(false);
        }
        if let Some(result) = compare(&left[i], &right[i]) {
            return Some(result);
        }
    }
    // vecs are the same
    None
}

fn compare(left: &ListOrInt, right: &ListOrInt) -> Option<bool> {
    match (left, right) {
        // compare ints
        (ListOrInt::Int(l), ListOrInt::Int(r)) => {
            if l < r {
                return Some(true);
            }
            if l > r {
                return Some(false);
            }
            None
        }
        // comare lists
        (ListOrInt::List(l), ListOrInt::List(r)) => compare_vec(&l, &r),
        // if one is a list, convert and re-compare
        (ListOrInt::Int(_), ListOrInt::List(r)) => {
            let mut new_vec = Vec::new();
            new_vec.push(left.clone());
            compare_vec(&new_vec, &r)
        }
        (ListOrInt::List(l), ListOrInt::Int(_)) => {
            let mut new_vec = Vec::new();
            new_vec.push(right.clone());
            compare_vec(&l, &new_vec)
        }
    }
}

pub fn part1(file_contents: String, _p: Option<Params>) -> String {
    //println!("{}", file_contents);
    let packet_pairs = parse_packets(&file_contents);

    let ordered: Vec<usize> = packet_pairs
        .iter()
        .enumerate()
        .filter_map(|(i, pair)| {
            //println!("{:?}", pair);
            if check_order(pair) {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect();
    //println!("{:?}", ordered);
    let sum: usize = ordered.iter().sum();
    format!("{}", sum)
}

pub fn part2(file_contents: String, _p: Option<Params>) -> String {
    //println!("{}", file_contents);
    let mut all_packets: Vec<Packet> = file_contents
        .lines()
        .filter(|l| l != &"")
        .map(|l| {
            let (leftover, p) = packet(l).expect("Could not parse packet");
            assert_eq!(leftover, "");
            Packet::new(p, l.to_string())
        })
        .collect();

    // add the 2 divider packets
    let (_, div1) = packet("[[2]]").expect("Could not parse divider [[2]]");
    all_packets.push(Packet::new(div1, "[[2]]".to_string()));
    let (_, div2) = packet("[[6]]").expect("Could not parse divider [[6]]");
    all_packets.push(Packet::new(div2, "[[6]]".to_string()));

    // sort in-place
    all_packets.sort_by(|a, b| match compare_vec(&a.parsed, &b.parsed) {
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
        None => Ordering::Equal,
    });

    for p in all_packets.iter() {
        println!("{:?}", p);
    }

    // find the indices of the divider packets
    let indices: Vec<usize> = all_packets
        .iter()
        .enumerate()
        .filter_map(|(i, p)| {
            if p.orig == "[[2]]" || p.orig == "[[6]]" {
                Some(i + 1)
            } else {
                None
            }
        })
        .collect();
    println!("{:?}", indices);

    format!("{}", indices.iter().product::<usize>())
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day13-example.txt");
        assert_eq!(part1(input, None), "13".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day13-input.txt");
        assert_eq!(part1(input, None), "6235".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day13-example.txt");
        assert_eq!(part2(input, None), "140".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day13-input.txt");
        assert_eq!(part2(input, None), "22866".to_string());
    }
}
