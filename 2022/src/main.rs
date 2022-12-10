use std::fs;

use clap::Parser;

mod cli;
mod days;

macro_rules! run_for_day {
    ($d:ident, $p:expr, $f:ident) => {{
        println!("Part {}", $p);
        match $p {
            cli::Part::One => days::$d::part1($f),
            cli::Part::Two => days::$d::part2($f),
        }
    }};
}
fn main() {
    let args = cli::Args::parse();
    println!("Day {}", args.day);

    // try to read the input file first
    println!("reading file {:?}", args.file);
    let file_contents = fs::read_to_string(args.file).expect("failed to read file");

    // validate days and parts
    let answer = match args.day {
        1 => run_for_day!(day1, args.part, file_contents),
        2 => run_for_day!(day2, args.part, file_contents),
        3 => run_for_day!(day3, args.part, file_contents),
        4 => run_for_day!(day4, args.part, file_contents),
        5 => run_for_day!(day5, args.part, file_contents),
        6 => run_for_day!(day6, args.part, file_contents),
        7 => run_for_day!(day7, args.part, file_contents),
        8 => run_for_day!(day8, args.part, file_contents),
        9..=25 => {
            println!("Day {}, part {}", args.day, args.part,);
            unimplemented!("Haven't done that day yet")
        }
        _ => panic!("Day {} is out of range", args.day),
    };

    println!("answer: {}", answer);
}
