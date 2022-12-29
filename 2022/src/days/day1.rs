use super::expect_usize;
use run_aoc::runner_fn;

#[runner_fn]
fn part1(file_contents: String) -> usize {
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

    max_value
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
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

    sum
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day1, part1, example, 24000);
    test_fn!(day1, part1, input, 67016);

    test_fn!(day1, part2, example, 45000);
    test_fn!(day1, part2, input, 200116);
}
