mod days;

fn main() {
    let args_test: Vec<String> = std::env::args().skip(1).collect();
    match args_test[0].as_str() {
        "run" => cli::run(&args_test[1..]),
        // TODO: download the description and input for the input day
        "download" => unimplemented!(),
        "-h" | "--help" => println!("help! TODO"),
        _ => panic!("unknown option {}", args_test[0]),
    }
}

mod cli {
    use std::collections::HashMap;
    use std::fmt;
    use std::fs;
    use std::str::FromStr;

    use nom::bytes::complete::tag;
    use nom::character::complete::alphanumeric1;
    use nom::multi::separated_list1;
    use nom::sequence::separated_pair;
    use nom::IResult;
    use seq_macro::seq;

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
                Part::One => write!(f, "1"),
                Part::Two => write!(f, "2"),
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
        // comma-separated list, like 'x=y,foo=bar'
        let params: Option<HashMap<String, String>> = params.map(|p| parse_params(p));
        // variation to use ('example', 'example2', 'input', etc.)
        //let input

        println!("Day {}", day);
        seq!(N in 1..=25 {
            let day_fn: fn(String, Option<HashMap<String, String>>) -> String = match day {
                #(
                    N => runner_fn_for_day!(day~N, part),
                )*
                // TODO: usage here?
                _ => panic!("Day {} is out of range", day),
            };
        });

        // try to read the input file
        let file_path = format!("inputs/day{}-{}.txt", day, input);
        println!("reading file '{}'", file_path);
        let file_contents = fs::read_to_string(file_path).expect("failed to read file");

        let answer = day_fn(file_contents, params);
        println!("\nanswer:\n{}", answer);
    }

    pub fn parse_params(list: &str) -> HashMap<String, String> {
        let (leftover, input_params) =
            separated_list1(tag(","), parse_pair)(list).expect("could not parse input params");
        assert_eq!(leftover, "");

        let mut params: HashMap<String, String> = HashMap::new();
        for (p, v) in input_params.into_iter() {
            params.insert(p.to_string(), v.to_string());
        }
        params
    }

    fn parse_pair(input: &str) -> IResult<&str, (&str, &str)> {
        separated_pair(alphanumeric1, tag("="), alphanumeric1)(input)
    }
}
