use clap::Parser;

mod cli;
mod days;

fn main() {
    let args = cli::Args::parse();

    // validate days and parts
    match args.day {
        1 => {
            println!("Day {}", args.day);
            days::day1::run(args.part, args.file);
        }
        2..=25 => {
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
