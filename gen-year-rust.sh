#!/usr/bin/env bash
# Generate the Rust layout for a year of AoC

usage() {
  echo ""
  echo ""
  echo "Usage:"
  echo "  ./gen-year-rust.sh <year>"
  echo ""
}

# arguments
year="${1:?"Missing the 'year' argument$(usage)"}"

# directory tree
echo "generating the directory tree..."
mkdir -p ./"$year"/{descriptions,inputs,src}

# Cargo.toml
echo "generating ./$year/Cargo.toml..."
cat <<CARGO_EOF > "./$year/Cargo.toml"
[package]
name = "aoc-$year"
version = "0.1.0"
edition = "2021"

[dependencies]
nom = "7"
run-aoc = { path = "../crates/run-aoc" }
seq-macro = "0.3"
utils = { path = "../crates/utils" }
CARGO_EOF
git add "./$year/Cargo.toml"


# README.md
echo "generating ./$year/README.md..."
cat <<README_EOF > "./$year/README.md"
# aoc-$year

Solutions for year $year

## Build and run a specific day

\`\`\`
$ cargo run -- run <day> <one|two> [params] <input type>
\`\`\`

No parameters needed (for example: day 3, part 2, example data)

\`\`\`
$ cargo run -- run 3 two example
\`\`\`

If parameters are needed (for example: day 15, part 1, input data)

\`\`\`
$ cargo run -- run 15 one y=10 input
\`\`\`

## Testing

\`\`\`
$ cargo test
\`\`\`
README_EOF
git add "./$year/README.md"


# main.rs
echo "generating ./$year/src/main.rs..."
cat <<MAIN_EOF > "./$year/src/main.rs"
use run_aoc::aoc_cli;
use seq_macro::seq;

seq!(N in 1..=25 {
    pub mod day~N;
});

aoc_cli!();
MAIN_EOF
git add "./$year/src/main.rs"


# day*.rs
echo "generating ./$year/src/day*.rs..."
for day in {1..25}
do
  cat <<DAY_EOF > "./$year/src/day$day.rs"
use run_aoc::runner_fn;

#[runner_fn]
fn part1(file_contents: String) -> usize {
    println!("{}", file_contents);
    0
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    println!("{}", file_contents);
    0
}

#[cfg(test)]
mod tests {
    // use run_aoc::test_fn;

    // test_fn!(day$day, part1, example, 0);
    // test_fn!(day$day, part1, input, 0);

    // test_fn!(day$day, part2, example, 0);
    // test_fn!(day$day, part2, input, 0);
}
DAY_EOF
done
git add ./"$year"/src/day*.rs


# run the build
echo "running 'cargo build'..."
cd "./$year" || exit
cargo build || exit
cd ..
git add "./$year/Cargo.lock"


# commit
git commit -m "generate skeleton for $year"
