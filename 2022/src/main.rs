use std::fs;

use clap::Parser;

mod cli;
mod days;

macro_rules! fn_for_day {
    ($d:ident, $p:expr) => {{
        println!("Part {}", $p);
        match $p {
            cli::Part::One => days::$d::part1,
            cli::Part::Two => days::$d::part2,
        }
    }};
}

fn main() {
    let args = cli::Args::parse();
    println!("Day {}", args.day);

    // validate days and parts
    let day_fn = match args.day {
        1 => fn_for_day!(day1, args.part),
        2 => fn_for_day!(day2, args.part),
        3 => fn_for_day!(day3, args.part),
        4 => fn_for_day!(day4, args.part),
        5 => fn_for_day!(day5, args.part),
        6 => fn_for_day!(day6, args.part),
        7 => fn_for_day!(day7, args.part),
        8 => fn_for_day!(day8, args.part),
        9 => fn_for_day!(day9, args.part),
        10 => fn_for_day!(day10, args.part),
        11 => fn_for_day!(day11, args.part),
        12 => fn_for_day!(day12, args.part),
        13 => fn_for_day!(day13, args.part),
        14 => fn_for_day!(day14, args.part),
        15 => fn_for_day!(day15, args.part),
        16 => fn_for_day!(day16, args.part),
        17 => fn_for_day!(day17, args.part),
        18 => fn_for_day!(day18, args.part),
        19 => fn_for_day!(day19, args.part),
        20 => fn_for_day!(day20, args.part),
        21 => fn_for_day!(day21, args.part),
        22 => fn_for_day!(day22, args.part),
        23 => fn_for_day!(day23, args.part),
        24 => fn_for_day!(day24, args.part),
        25 => fn_for_day!(day25, args.part),
        _ => panic!("Day {} is out of range", args.day),
    };

    // try to read the input file
    println!("reading file {:?}", args.file);
    let file_contents = fs::read_to_string(args.file).expect("failed to read file");

    let answer = day_fn(file_contents);
    println!("answer:\n{}", answer);
}
