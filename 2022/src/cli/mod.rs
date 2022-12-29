use std::fmt;

use clap::{Parser, ValueEnum};
use nom::bytes::complete::tag;
use nom::character::complete::alphanumeric1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

#[derive(Parser)]
#[command(author, version, about = "Advent of Code 2022", long_about = None)]
pub struct Args {
    /// Which day is this (1-25)
    #[arg(short, long)]
    pub day: u8,

    /// Which part of the problem does this solve
    #[arg(short, long, value_enum)]
    pub part: Part,

    /// Optional params to pass to the solver (comma-separated list, like 'x=y,foo=bar')
    #[arg(long)]
    pub params: Option<String>,

    /// Variation to use ('example', 'example2', 'input', etc.)
    pub variation: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Part {
    /// The first part of the day's problem
    One,
    /// The second (harder) part of the problem, unlocked after solving the first part
    Two,
}

impl fmt::Display for Part {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Part::One => write!(f, "1"),
            Part::Two => write!(f, "2"),
        }
    }
}

pub struct Params {
    params: Vec<(String, String)>,
}

impl Params {
    pub fn from(list: &str) -> Self {
        let (leftover, input_params) = separated_list1(tag(","), Params::parse_pair)(list)
            .expect("could not parse input params");
        assert_eq!(leftover, "");

        let params: Vec<(String, String)> = input_params
            .into_iter()
            .map(|(p, v)| (p.to_string(), v.to_string()))
            .collect();
        Params { params }
    }

    fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(alphanumeric1, tag("="), alphanumeric1)(input)
    }

    pub fn get(&self, param: &str) -> Option<String> {
        for (p, v) in self.params.iter() {
            if p == param {
                return Some(v.clone());
            }
        }
        None
    }
}
