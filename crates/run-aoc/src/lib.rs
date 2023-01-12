extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{Ident, Literal, Span, TokenTree};
use quote::quote;
use syn::{parse_macro_input, ItemFn};

// create a runner function to wrap the input function and Display its output
#[proc_macro_attribute]
pub fn runner_fn(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let original_fn = parse_macro_input!(input as ItemFn);
    // this contains the function name, and params
    let signature = original_fn.sig.clone();

    // function name
    let ident = signature.ident;
    let ident_name = format!("{}", ident);

    let runner_name = if ident_name == "part1" || ident_name == "part2" {
        Ident::new(&format!("__{}_runner", ident_name), Span::call_site())
    } else {
        return syn::Error::new(
            ident.span(),
            "runner function must be named 'part1' or 'part2'",
        )
        .to_compile_error()
        .into();
    };

    let num_args = signature.inputs.len();

    match num_args {
        1 => TokenStream::from(quote!(
            #original_fn

            // only takes one arg, doesn't expect params
            pub fn #runner_name(file_contents: String, _p: Option<std::collections::HashMap<String, String>>) -> String {
                let result = #ident(file_contents);
                format!("{}", result)
            }
        )),
        2 => TokenStream::from(quote!(
            #original_fn

            pub fn #runner_name(file_contents: String, p: Option<std::collections::HashMap<String, String>>) -> String {
                let result = #ident(file_contents, p);
                format!("{}", result)
            }
        )),
        _ => syn::Error::new(
            signature.paren_token.span,
            format!(
                "runner functions take 1 or 2 arguments, found {}",
                signature.inputs.len()
            ),
        )
        .to_compile_error()
        .into(),
    }
}

// generate a test function, for making sure things work when I do refactorings and such
#[proc_macro]
pub fn test_fn(input: TokenStream) -> TokenStream {
    let input = proc_macro2::TokenStream::from(input);
    // collect and remove the punctuation, doesn't matter
    let tt: Vec<TokenTree> = input
        .into_iter()
        .filter(|tt| match tt {
            TokenTree::Punct(_) => false,
            _ => true,
        })
        .collect();

    if tt.len() == 4 {
        // no params
        // ex: aoc_test!(day1, part1, example, 24000);
        match (&tt[0], &tt[1], &tt[2], &tt[3]) {
            (
                TokenTree::Ident(day),
                TokenTree::Ident(part),
                TokenTree::Ident(variation),
                TokenTree::Literal(expected),
            ) => {
                let test_name = Ident::new(&format!("{}_{}", part, variation), Span::call_site());
                let file = format!("inputs/{}-{}.txt", day, variation);
                let file_name = Literal::string(&file);
                let fail_literal = Literal::string(&format!("failed to read file '{}'", file));
                let part_fn = Ident::new(&format!("{}", part), Span::call_site());
                TokenStream::from(quote!(
                    #[test]
                    fn #test_name() {
                        let file = #file_name;
                        let input = std::fs::read_to_string(&file).expect(#fail_literal);
                        assert_eq!(super::#part_fn(input), #expected);
                    }
                ))
            }
            _ => syn::Error::new(
                Span::call_site(),
                "expected args of type (ident, ident, ident, literal), found something else",
            )
            .to_compile_error()
            .into(),
        }
    } else if tt.len() == 5 {
        // with params
        match (&tt[0], &tt[1], &tt[2], &tt[3], &tt[4]) {
            (
                TokenTree::Ident(day),
                TokenTree::Ident(part),
                TokenTree::Ident(variation),
                TokenTree::Literal(params),
                TokenTree::Literal(expected)
            ) => {
                let test_name = Ident::new(
                    &format!("{}_{}", part, variation),
                    Span::call_site(),
                );
                let file = format!("inputs/{}-{}.txt", day, variation);
                let file_name = Literal::string(&file);
                let fail_literal = Literal::string(&format!(
                    "failed to read file '{}'",
                    file
                ));
                let part_fn = Ident::new(&format!("{}", part), Span::call_site());

                TokenStream::from(quote!(
                    #[test]
                    fn #test_name() {
                        let file = #file_name;
                        let params = crate::cli::parse_params(#params);
                        let input = std::fs::read_to_string(&file).expect(#fail_literal);
                        assert_eq!(super::#part_fn(input, Some(params)), #expected);
                    }
                ))
            },
            _ => syn::Error::new(
                Span::call_site(),
                "expected args of type (ident, ident, ident, literal, literal), found something else",
            )
                .to_compile_error()
                .into()
        }
    } else {
        syn::Error::new(
            Span::call_site(),
            format!("this macro takes 4 or 5 arguments, found {}", tt.len()),
        )
        .to_compile_error()
        .into()
    }
}

// generate the CLI
// TODO: currently no args to this, but will need the year for auto-downloading
#[proc_macro]
pub fn aoc_cli(_input: TokenStream) -> TokenStream {
    TokenStream::from(quote!(
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
                        Part::One => super::$d::__part1_runner,
                        Part::Two => super::$d::__part2_runner,
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
                println!("Params: {:?}", params);

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
    ))
}
