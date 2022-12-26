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
    use crate::days::test::aoc_test;

    aoc_test!(part1_example: "day1", part1, "example", 24000);
    aoc_test!(part1_input: "day1", part1, "input", 67016);

    aoc_test!(part2_example: "day1", part2, "example", 45000);
    aoc_test!(part2_input: "day1", part2, "input", 200116);
}
