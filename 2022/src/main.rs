use std::collections::HashMap;
use std::fs;

use clap::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use seq_macro::seq;

mod cli;
mod days;

macro_rules! runner_fn_for_day {
    ($d:ident, $p:expr) => {{
        println!("Part {}", $p);
        match $p {
            cli::Part::One => days::$d::__part1_runner,
            cli::Part::Two => days::$d::__part2_runner,
        }
    }};
}

fn main() {
    let args = cli::Args::parse();
    println!("Day {}", args.day);

    // validate days and parts
    seq!(N in 1..=25 {
        let day_fn: fn(String, Option<HashMap<String, String>>) -> String = match args.day {
            #(
                N => runner_fn_for_day!(day~N, args.part),
            )*
            _ => panic!("Day {} is out of range", args.day),
        };
    });

    // input params
    let params = match args.params {
        Some(list) => Some(parse_params(&list)),
        None => None,
    };

    // try to read the input file
    let file_path = format!("inputs/day{}-{}.txt", args.day, args.variation);
    println!("reading file '{}'", file_path);
    let file_contents = fs::read_to_string(file_path).expect("failed to read file");

    let answer = day_fn(file_contents, params);
    println!("\nanswer:\n{}", answer);
}

pub(crate) fn parse_params(list: &str) -> HashMap<String, String> {
    let (leftover, input_params) =
        separated_list1(tag(","), parse_pair)(list).expect("could not parse input params");
    assert_eq!(leftover, "");

    let mut params: HashMap<String, String> = HashMap::new();
    for (p, v) in input_params.into_iter() {
        params.insert(p.to_string(), v.to_string());
    }
    params
}

fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
    separated_pair(alphanumeric1, tag("="), alphanumeric1)(input)
}
