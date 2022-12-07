use clap::Parser;

mod cli;
mod days;

fn main() {
    let args = cli::Args::parse();
    println!("Day {}", args.day);

    // validate days and parts
    match args.day {
        1 => days::day1::run(args.part, args.file),
        2 => days::day2::run(args.part, args.file),
        3 => days::day3::run(args.part, args.file),
        4 => days::day4::run(args.part, args.file),
        5..=25 => {
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
