// re-export the proc_macro stuff
pub use aoc_proc::runner_fn;
pub use aoc_proc::test_fn;

pub mod cli;
pub mod download;
mod parse;

// generate functions & macros needed in main.rs
#[macro_export]
macro_rules! aoc_cli {
    ($year:literal) => {
        fn main() {
            let args_test: Vec<String> = std::env::args().skip(1).collect();
            let year = $year;
            println!("Year {}", year);
            handle_args(args_test, year).unwrap_or_else(|err| {
                println!("Error: {}", err);
                // TODO: use a custom error type, and only show usage is applicable
                run_aoc::cli::usage();
            });
        }

        fn handle_args(args: Vec<String>, year: usize) -> Result<(), String> {
            match args[0].as_str() {
                "run" => {
                    let parsed_args = run_aoc::cli::parse_run_args(&args[1..])?;
                    let day_fn = fn_for_day(parsed_args.0, parsed_args.1)?;
                    // TODO: maybe just show the error, but don't fail?
                    // (because then it prints usage, which is not great)
                    run_aoc::download::auto_download(year, parsed_args.0)?;
                    run_aoc::cli::run_day_fn(day_fn, parsed_args.2, parsed_args.3)?;
                    Ok(())
                }
                "html" => {
                    let (day, force) = run_aoc::cli::parse_html_args(&args[1..])?;
                    run_aoc::download::dl_html(year, day, force)?;
                    Ok(())
                }
                "md" => {
                    let (day, force) = run_aoc::cli::parse_md_args(&args[1..])?;
                    run_aoc::download::dl_md(year, day, force)?;
                    Ok(())
                }
                "help" | "-h" | "--help" => Ok(run_aoc::cli::usage()),
                _ => Err(format!("unknown sub-command '{}'", args[0])),
            }
        }

        macro_rules! runner_fn_for_day {
            ($d:ident, $p:expr) => {{
                match $p {
                    run_aoc::cli::Part::One => Ok($d::__part1_runner),
                    run_aoc::cli::Part::Two => Ok($d::__part2_runner),
                }
            }};
        }

        fn fn_for_day(
            day: usize,
            part: run_aoc::cli::Part,
        ) -> Result<fn(String, Option<run_aoc::cli::Params>) -> String, String> {
            println!("Day {}, part {}", day, part);
            match day {
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
                _ => Err(format!("Day {} is out of range", day)),
            }
        }
    };
}
