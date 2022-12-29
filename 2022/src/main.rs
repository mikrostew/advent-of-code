use std::fs;

use clap::Parser;

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
    // TODO: can I use seq! macro for this?
    let day_fn: fn(String, Option<cli::Params>) -> String = match args.day {
        1 => runner_fn_for_day!(day1, args.part),
        2 => runner_fn_for_day!(day2, args.part),
        3 => runner_fn_for_day!(day3, args.part),
        4 => runner_fn_for_day!(day4, args.part),
        5 => runner_fn_for_day!(day5, args.part),
        6 => runner_fn_for_day!(day6, args.part),
        7 => runner_fn_for_day!(day7, args.part),
        8 => runner_fn_for_day!(day8, args.part),
        9 => runner_fn_for_day!(day9, args.part),
        10 => runner_fn_for_day!(day10, args.part),
        11 => runner_fn_for_day!(day11, args.part),
        12 => runner_fn_for_day!(day12, args.part),
        13 => runner_fn_for_day!(day13, args.part),
        14 => runner_fn_for_day!(day14, args.part),
        15 => runner_fn_for_day!(day15, args.part),
        16 => runner_fn_for_day!(day16, args.part),
        17 => runner_fn_for_day!(day17, args.part),
        18 => runner_fn_for_day!(day18, args.part),
        19 => runner_fn_for_day!(day19, args.part),
        20 => runner_fn_for_day!(day20, args.part),
        21 => runner_fn_for_day!(day21, args.part),
        22 => runner_fn_for_day!(day22, args.part),
        23 => runner_fn_for_day!(day23, args.part),
        24 => runner_fn_for_day!(day24, args.part),
        25 => runner_fn_for_day!(day25, args.part),
        _ => panic!("Day {} is out of range", args.day),
    };

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
