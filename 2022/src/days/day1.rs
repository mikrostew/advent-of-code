use super::expect_usize;
use crate::cli::Params;

pub fn part1(file_contents: String, _p: Option<Params>) -> String {
    let mut current_total: usize = 0;
    let mut max_value: usize = 0;

    file_contents.lines().for_each(|line| match line {
        "" => {
            println!("(empty)");
            if current_total > max_value {
                max_value = current_total;
            }
            current_total = 0;
        }
        _ => {
            println!("line: {}", line);
            let as_int = expect_usize!(line);
            current_total += as_int;
        }
    });

    // account for not getting an empty line at the end
    if current_total != 0 {
        if current_total > max_value {
            max_value = current_total;
        }
    }

    println!("");
    println!("max value: {}", max_value);
    format!("{}", max_value)
}

pub fn part2(file_contents: String, _p: Option<Params>) -> String {
    let mut current_total: usize = 0;
    let mut totals: Vec<usize> = vec![];

    file_contents.lines().for_each(|line| match line {
        "" => {
            println!("(empty)");
            totals.push(current_total);
            current_total = 0;
        }
        _ => {
            println!("line: {}", line);
            let as_int = expect_usize!(line);
            current_total += as_int;
        }
    });

    // account for not getting an empty line at the end
    if current_total != 0 {
        totals.push(current_total);
    }

    // get total of the top 3
    totals.sort();
    totals.reverse();
    let sum: usize = totals[0..=2].iter().sum();

    println!("");
    println!("total of top 3: {:?}", sum);
    format!("{}", sum)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day1-example.txt");
        assert_eq!(part1(input, None), "24000".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day1-input.txt");
        assert_eq!(part1(input, None), "67016".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day1-example.txt");
        assert_eq!(part2(input, None), "45000".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day1-input.txt");
        assert_eq!(part2(input, None), "200116".to_string());
    }
}
