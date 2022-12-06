use std::path::Path;

use super::{read_file, run_parts};

run_parts!();

enum Choice {
    Rock,
    Paper,
    Scissors,
}

fn opponent_str_to_choice(character: &str) -> Choice {
    match character {
        "A" => Choice::Rock,
        "B" => Choice::Paper,
        "C" => Choice::Scissors,
        _ => panic!("Don't recognize character {}", character),
    }
}

fn my_str_to_choice(character: &str) -> Choice {
    match character {
        "X" => Choice::Rock,
        "Y" => Choice::Paper,
        "Z" => Choice::Scissors,
        _ => panic!("Don't recognize character {}", character),
    }
}

fn my_choice_based_on_outcome(opponent_choice: &Choice, character: &str) -> Choice {
    match (opponent_choice, character) {
        // lose
        (Choice::Rock, "X") => Choice::Scissors,
        (Choice::Paper, "X") => Choice::Rock,
        (Choice::Scissors, "X") => Choice::Paper,
        // draw
        (Choice::Rock, "Y") => Choice::Rock,
        (Choice::Paper, "Y") => Choice::Paper,
        (Choice::Scissors, "Y") => Choice::Scissors,
        // win
        (Choice::Rock, "Z") => Choice::Paper,
        (Choice::Paper, "Z") => Choice::Scissors,
        (Choice::Scissors, "Z") => Choice::Rock,
        (_, _) => panic!("Don't recognize character {}", character),
    }
}

fn score_for_round(round_str: &str) -> i32 {
    let opponent_choice = opponent_str_to_choice(&round_str[0..1]);
    let my_choice = my_str_to_choice(&round_str[2..3]);
    score_for_choice(&my_choice) + score_for_outcome(my_choice, opponent_choice)
}

fn score_for_round_2(round_str: &str) -> i32 {
    let opponent_choice = opponent_str_to_choice(&round_str[0..1]);
    let my_choice = my_choice_based_on_outcome(&opponent_choice, &round_str[2..3]);
    score_for_choice(&my_choice) + score_for_outcome(my_choice, opponent_choice)
}

fn score_for_choice(choice: &Choice) -> i32 {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn score_for_outcome(my_choice: Choice, opponent_choice: Choice) -> i32 {
    match (my_choice, opponent_choice) {
        // win
        (Choice::Rock, Choice::Scissors) => 6,
        (Choice::Paper, Choice::Rock) => 6,
        (Choice::Scissors, Choice::Paper) => 6,
        // loss
        (Choice::Rock, Choice::Paper) => 0,
        (Choice::Paper, Choice::Scissors) => 0,
        (Choice::Scissors, Choice::Rock) => 0,
        // draw
        (Choice::Rock, Choice::Rock) => 3,
        (Choice::Paper, Choice::Paper) => 3,
        (Choice::Scissors, Choice::Scissors) => 3,
    }
}

fn part1<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);

    let total_score: i32 = file_contents
        .lines()
        .map(|line| score_for_round(line))
        .sum();

    println!("Total score: {}", total_score);
}

fn part2<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);

    let total_score: i32 = file_contents
        .lines()
        .map(|line| score_for_round_2(line))
        .sum();

    println!("Total score: {}", total_score);
}
