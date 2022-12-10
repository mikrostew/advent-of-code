use super::run_parts;

run_parts!();

fn part1(file_contents: String) -> () {
    let mut current_total: i32 = 0;
    let mut max_value: i32 = 0;

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
            let as_int = line.parse::<i32>().expect("failed to parse int");
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
}

fn part2(file_contents: String) -> () {
    let mut current_total: i32 = 0;
    let mut totals: Vec<i32> = vec![];

    file_contents.lines().for_each(|line| match line {
        "" => {
            println!("(empty)");
            totals.push(current_total);
            current_total = 0;
        }
        _ => {
            println!("line: {}", line);
            let as_int = line.parse::<i32>().expect("failed to parse int");
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
    let sum: i32 = totals[0..=2].iter().sum();

    println!("");
    println!("total of top 3: {:?}", sum);
}
