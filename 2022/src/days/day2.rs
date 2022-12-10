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

fn score_for_round(round_str: &str) -> usize {
    let opponent_choice = opponent_str_to_choice(&round_str[0..1]);
    let my_choice = my_str_to_choice(&round_str[2..3]);
    score_for_choice(&my_choice) + score_for_outcome(my_choice, opponent_choice)
}

fn score_for_round_2(round_str: &str) -> usize {
    let opponent_choice = opponent_str_to_choice(&round_str[0..1]);
    let my_choice = my_choice_based_on_outcome(&opponent_choice, &round_str[2..3]);
    score_for_choice(&my_choice) + score_for_outcome(my_choice, opponent_choice)
}

fn score_for_choice(choice: &Choice) -> usize {
    match choice {
        Choice::Rock => 1,
        Choice::Paper => 2,
        Choice::Scissors => 3,
    }
}

fn score_for_outcome(my_choice: Choice, opponent_choice: Choice) -> usize {
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

pub fn part1(file_contents: String) -> String {
    let total_score: usize = file_contents
        .lines()
        .map(|line| score_for_round(line))
        .sum();

    println!("Total score: {}", total_score);
    format!("{}", total_score)
}

pub fn part2(file_contents: String) -> String {
    let total_score: usize = file_contents
        .lines()
        .map(|line| score_for_round_2(line))
        .sum();

    println!("Total score: {}", total_score);
    format!("{}", total_score)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day2-example.txt");
        assert_eq!(part1(input), "15".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day2-input.txt");
        assert_eq!(part1(input), "13005".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day2-example.txt");
        assert_eq!(part2(input), "12".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day2-input.txt");
        assert_eq!(part2(input), "11373".to_string());
    }
}
