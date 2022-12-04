use clap::Parser;
use std::path;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about = "Advent of Code 2022", long_about = None)]
struct Args {
    /// Which day is this?
    #[arg(short, long)]
    day: u8,

    /// Which part is this?
    #[arg(short, long)]
    part: u8,

    /// File to read
    file: path::PathBuf,
}

fn main() {
    let args = Args::parse();

    // validate days and parts
    match args.day {
        1..=25 => {
            println!(
                "Day {}, part {}, reading file '{}'",
                args.day,
                args.part,
                args.file.display()
            );
            unimplemented!("Haven't done that day yet")
        }
        _ => println!("Day {} is out of range", args.day),
    }
}
