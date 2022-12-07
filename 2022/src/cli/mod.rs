use std::fmt;
use std::path;

use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(author, version, about = "Advent of Code 2022", long_about = None)]
pub struct Args {
    /// Which day is this (1-25)
    #[arg(short, long)]
    pub day: u8,

    /// Which part of the problem does this solve
    #[arg(short, long, value_enum)]
    pub part: Part,

    /// File to read
    pub file: path::PathBuf,
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
