use run_aoc::aoc_cli;
use seq_macro::seq;

seq!(N in 1..=25 {
    pub mod day~N;
});

aoc_cli!();
