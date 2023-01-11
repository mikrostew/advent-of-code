mod days;

fn main() {
    let args_test: Vec<String> = std::env::args().skip(1).collect();
    match args_test[0].as_str() {
        "run" => cli::run(&args_test[1..]),
        // TODO: download the description and input for the input day
        "download" => unimplemented!(),
        "-h" | "--help" => println!("help! TODO"),
        _ => panic!("unknown option '{}'", args_test[0]),
    }
}

mod cli {
    use std::collections::HashMap;
    use std::{fmt, fs, str::FromStr};

    macro_rules! runner_fn_for_day {
        ($d:ident, $p:expr) => {{
            println!("Part {}", $p);
            match $p {
                Part::One => super::days::$d::__part1_runner,
                Part::Two => super::days::$d::__part2_runner,
            }
        }};
    }

    pub enum Part {
        One,
        Two,
    }

    impl FromStr for Part {
        type Err = ();

        fn from_str(input: &str) -> Result<Self, Self::Err> {
            match input {
                "one" | "One" => Ok(Part::One),
                "two" | "Two" => Ok(Part::Two),
                _ => Err(()),
            }
        }
    }

    impl fmt::Display for Part {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Part::One => write!(f, "one"),
                Part::Two => write!(f, "two"),
            }
        }
    }

    pub fn run(args: &[String]) {
        println!("run args: {:?}", args);
        match args.len() {
            3 => run_for_day(&args[0], &args[1], None, &args[2]),
            4 => run_for_day(&args[0], &args[1], Some(&args[2]), &args[3]),
            _ => unimplemented!("help/usage is TODO"),
        }
    }

    fn run_for_day(day: &str, part: &str, params: Option<&str>, input: &str) {
        let day = day
            .parse::<usize>()
            .unwrap_or_else(|_| panic!("could not parse day '{}' as a number", day));
        let part: Part = part.parse().expect("could not parse part one or two");
        let params: Option<HashMap<String, String>> = params.map(|p| parse_params(p));

        println!("Day {}", day);
        let day_fn: fn(String, Option<HashMap<String, String>>) -> String = match day {
            1 => runner_fn_for_day!(day1, part),
            2 => runner_fn_for_day!(day2, part),
            3 => runner_fn_for_day!(day3, part),
            4 => runner_fn_for_day!(day4, part),
            5 => runner_fn_for_day!(day5, part),
            6 => runner_fn_for_day!(day6, part),
            7 => runner_fn_for_day!(day7, part),
            8 => runner_fn_for_day!(day8, part),
            9 => runner_fn_for_day!(day9, part),
            10 => runner_fn_for_day!(day10, part),
            11 => runner_fn_for_day!(day11, part),
            12 => runner_fn_for_day!(day12, part),
            13 => runner_fn_for_day!(day13, part),
            14 => runner_fn_for_day!(day14, part),
            15 => runner_fn_for_day!(day15, part),
            16 => runner_fn_for_day!(day16, part),
            17 => runner_fn_for_day!(day17, part),
            18 => runner_fn_for_day!(day18, part),
            19 => runner_fn_for_day!(day19, part),
            20 => runner_fn_for_day!(day20, part),
            21 => runner_fn_for_day!(day21, part),
            22 => runner_fn_for_day!(day22, part),
            23 => runner_fn_for_day!(day23, part),
            24 => runner_fn_for_day!(day24, part),
            25 => runner_fn_for_day!(day25, part),
            // TODO: also usage here
            _ => panic!("Day {} is out of range", day),
        };

        // try to read the input file
        let file_path = format!("inputs/day{}-{}.txt", day, input);
        println!("reading file '{}'", file_path);
        let file_contents = fs::read_to_string(file_path).expect("failed to read file");

        let answer = day_fn(file_contents, params);
        println!("\nanswer:\n{}", answer);
    }

    // input is comma-separated list, like 'x=y,foo=bar'
    pub fn parse_params(list: &str) -> HashMap<String, String> {
        let mut params: HashMap<String, String> = HashMap::new();
        for param in list.split(",") {
            let pair: Vec<&str> = param.split("=").collect();
            if pair.len() != 2 {
                // TODO: also usage
                panic!(
                    "could not parse param '{}', expecing equal-separated pair like 'y=10'",
                    param
                );
            }
            params.insert(pair[0].to_string(), pair[1].to_string());
        }
        params
    }
}
