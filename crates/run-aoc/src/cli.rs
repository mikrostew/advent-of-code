use std::collections::HashMap;
use std::{fmt, fs, str::FromStr};

pub fn usage() {
    println!(
        "
Usage:

  RUN a specific day:
    cargo run -- run <1-25> <one|two> [params] <input-type>

    Options:
        params     comma-separated list of param pairs, e.g. 'x=2,max=56'

  DOWNLOAD input & description for a specific day:
    cargo run -- download <1-25>

  HELP
    cargo run -- help
    cargo run -- --help
    cargo run -- -h
"
    );
}

pub enum Part {
    One,
    Two,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "one" | "One" => Ok(Part::One),
            "two" | "Two" => Ok(Part::Two),
            _ => Err(format!("expected part 'one' or 'two', found '{}'", input)),
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

#[derive(Debug)]
pub struct Params {
    params: HashMap<String, String>,
}

impl Params {
    pub fn get(&self, param: &str) -> String {
        self.params
            .get(param)
            .unwrap_or_else(|| panic!("Could not get param {}", param))
            .clone()
    }
}

impl FromStr for Params {
    type Err = String;

    // input is comma-separated list, like 'x=y,foo=bar'
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut params: HashMap<String, String> = HashMap::new();
        for param in input.split(",") {
            let pair: Vec<&str> = param.split("=").collect();
            if pair.len() != 2 {
                return Err(format!(
                    "could not parse param '{}', expecing equal-separated pair like 'y=10'",
                    param
                ));
            }
            params.insert(pair[0].to_string(), pair[1].to_string());
        }
        Ok(Params { params })
    }
}

pub fn parse_args(args: &[String]) -> Result<(usize, Part, Option<Params>, String), String> {
    match args.len() {
        3 => args_for_day(&args[0], &args[1], None, &args[2]),
        4 => args_for_day(&args[0], &args[1], Some(&args[2]), &args[3]),
        _ => Err(format!(
            "expected 3 or 4 args to 'run', found {}",
            args.len()
        )),
    }
}

fn args_for_day(
    day: &str,
    part: &str,
    params: Option<&str>,
    input: &str,
) -> Result<(usize, Part, Option<Params>, String), String> {
    let day = day
        .parse::<usize>()
        .unwrap_or_else(|_| panic!("could not parse day '{}' as a number", day));
    let part: Part = part.parse()?;
    let params: Option<Params> = match params {
        Some(p) => {
            let parsed = p.parse()?;
            Some(parsed)
        }
        None => None,
    };
    let file_path = format!("inputs/day{}-{}.txt", day, input);
    Ok((day, part, params, file_path))
}

pub fn run_day_fn(
    day_fn: fn(String, Option<Params>) -> String,
    params: Option<Params>,
    file_path: String,
) -> Result<(), String> {
    println!("Params: {:?}", params);
    println!("reading file '{}'", file_path);
    let file_contents = match fs::read_to_string(file_path) {
        Ok(s) => s,
        Err(err) => {
            let err_str = if let Some(inner_err) = err.into_inner() {
                format!("{inner_err}")
            } else {
                format!("Some std::io::Error happened")
            };
            return Err(format!("Failed to read file: {err_str}"));
        }
    };

    let answer = day_fn(file_contents, params);
    println!("\nanswer:\n{}", answer);
    Ok(())
}
