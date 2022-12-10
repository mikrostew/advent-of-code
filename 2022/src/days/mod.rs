pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

macro_rules! run_parts {
    // no args to this
    () => {
        use crate::cli::Part;
        pub fn run(part: Part, file_contents: String) -> () {
            println!("Part {}", part);
            match part {
                Part::One => part1(file_contents),
                Part::Two => part2(file_contents),
            }
        }
    };
}

// who needs error handling, this ain't production code
macro_rules! parse_usize {
    ($e:ident) => {
        $e.parse::<usize>().expect("failed to parse usize!")
    }
}

pub(crate) use parse_usize;
pub(crate) use run_parts;
