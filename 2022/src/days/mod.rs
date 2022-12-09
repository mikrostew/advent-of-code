pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;

macro_rules! run_parts {
    // no args to this
    () => {
        use crate::cli::Part;
        pub fn run<P: AsRef<Path>>(part: Part, path: P) -> () {
            println!("Part {}", part);
            match part {
                Part::One => part1(path),
                Part::Two => part2(path),
            }
        }
    };
}

macro_rules! read_file {
    ($f:ident, $p:ident) => {
        let $f = std::fs::read_to_string($p).expect("failed to read file");
    };
}

// who needs error handling, this ain't production code
macro_rules! parse_usize {
    ($e:ident) => {
        $e.parse::<usize>().expect("failed to parse usize!")
    }
}

pub(crate) use parse_usize;
pub(crate) use read_file;
pub(crate) use run_parts;
