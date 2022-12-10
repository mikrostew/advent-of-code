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
        9 => run_for_day!(day9, args.part, file_contents),
        10 => run_for_day!(day10, args.part, file_contents),
        11 => run_for_day!(day11, args.part, file_contents),
        12 => run_for_day!(day12, args.part, file_contents),
        13 => run_for_day!(day13, args.part, file_contents),
        14 => run_for_day!(day14, args.part, file_contents),
        15 => run_for_day!(day15, args.part, file_contents),
        16 => run_for_day!(day16, args.part, file_contents),
        17 => run_for_day!(day17, args.part, file_contents),
        18 => run_for_day!(day18, args.part, file_contents),
        19 => run_for_day!(day19, args.part, file_contents),
        20 => run_for_day!(day20, args.part, file_contents),
        21 => run_for_day!(day21, args.part, file_contents),
        22 => run_for_day!(day22, args.part, file_contents),
        23 => run_for_day!(day23, args.part, file_contents),
        24 => run_for_day!(day24, args.part, file_contents),
        25 => run_for_day!(day25, args.part, file_contents),
        _ => panic!("Day {} is out of range", args.day),
    };

    println!("answer: {}", answer);
}
