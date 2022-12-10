use std::fs;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;

// who needs error handling, this ain't production code
macro_rules! parse_usize {
    ($e:ident) => {
        $e.parse::<usize>().expect("failed to parse usize!")
    }
}
pub(crate) use parse_usize;

// test helpers

fn read_input_file(path: &str) -> String {
    let file_contents = fs::read_to_string(path).expect("failed to read file");
    file_contents
}
