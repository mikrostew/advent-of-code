pub mod day1;
pub mod day2;

macro_rules! run_parts {
    // no args to this
    () => {
        use crate::cli::Part;
        pub fn run<P: AsRef<Path> + Debug>(part: Part, path: P) -> () {
            println!("Part {}", part);
            match part {
                Part::One => part1(path),
                Part::Two => part2(path),
            }
        }
    };
}

pub(crate) use run_parts;
