use std::fs;

use clap::Parser;

mod cli;
mod days;

fn main() {
    let args = cli::Args::parse();
    println!("Day {}", args.day);

    // try to read the input file first
    println!("reading file {:?}", args.file);
    let file_contents = fs::read_to_string(args.file).expect("failed to read file");

    // validate days and parts
    match args.day {
        1 => days::day1::run(args.part, file_contents),
        2 => days::day2::run(args.part, file_contents),
        3 => days::day3::run(args.part, file_contents),
        4 => days::day4::run(args.part, file_contents),
        5 => days::day5::run(args.part, file_contents),
        6 => days::day6::run(args.part, file_contents),
        7 => days::day7::run(args.part, file_contents),
        8 => days::day8::run(args.part, file_contents),
        9..=25 => {
            println!("Day {}, part {}", args.day, args.part,);
            unimplemented!("Haven't done that day yet")
        }
        _ => println!("Day {} is out of range", args.day),
    }
}
