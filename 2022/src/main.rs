use std::fs;

use clap::Parser;
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
        let day_fn: fn(String, Option<cli::Params>) -> String = match args.day {
            #(
                N => runner_fn_for_day!(day~N, args.part),
            )*
            _ => panic!("Day {} is out of range", args.day),
        };
    });

    // input params
    let params = match args.params {
        Some(list) => Some(cli::Params::from(&list)),
        None => None,
    };

    // try to read the input file
    let file_path = format!("inputs/day{}-{}.txt", args.day, args.variation);
    println!("reading file '{}'", file_path);
    let file_contents = fs::read_to_string(file_path).expect("failed to read file");

    let answer = day_fn(file_contents, params);
    println!("\nanswer:\n{}", answer);
}
