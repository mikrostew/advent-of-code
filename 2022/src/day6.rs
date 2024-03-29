use std::collections::HashSet;

use run_aoc::runner_fn;

// 4 distinct chars
fn find_start_of_packet_marker(line: &str) -> usize {
    for end_pos in 4..line.len() {
        let c1 = &line[(end_pos - 4)..(end_pos - 3)];
        let c2 = &line[(end_pos - 3)..(end_pos - 2)];
        let c3 = &line[(end_pos - 2)..(end_pos - 1)];
        let c4 = &line[(end_pos - 1)..(end_pos - 0)];
        println!("try: {}{}{}{}, pos: {}", c1, c2, c3, c4, end_pos);

        // if all chars are different, return the current position
        if c2 != c1 && c3 != c1 && c3 != c2 && c4 != c1 && c4 != c2 && c4 != c3 {
            return end_pos;
        }
    }
    panic!("start of packet marker not found");
}

// 14 distinct chars
fn find_start_of_message_marker(line: &str) -> usize {
    for end_pos in 14..line.len() {
        let c1 = &line[(end_pos - 14)..(end_pos - 13)];
        let c2 = &line[(end_pos - 13)..(end_pos - 12)];
        let c3 = &line[(end_pos - 12)..(end_pos - 11)];
        let c4 = &line[(end_pos - 11)..(end_pos - 10)];
        let c5 = &line[(end_pos - 10)..(end_pos - 9)];
        let c6 = &line[(end_pos - 9)..(end_pos - 8)];
        let c7 = &line[(end_pos - 8)..(end_pos - 7)];
        let c8 = &line[(end_pos - 7)..(end_pos - 6)];
        let c9 = &line[(end_pos - 6)..(end_pos - 5)];
        let c10 = &line[(end_pos - 5)..(end_pos - 4)];
        let c11 = &line[(end_pos - 4)..(end_pos - 3)];
        let c12 = &line[(end_pos - 3)..(end_pos - 2)];
        let c13 = &line[(end_pos - 2)..(end_pos - 1)];
        let c14 = &line[(end_pos - 1)..(end_pos - 0)];
        println!(
            "try: {}{}{}{}{}{}{}{}{}{}{}{}{}{}, pos: {}",
            c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, end_pos
        );

        // if all chars are different, return the current position
        let mut char_set: HashSet<&str> = HashSet::new();
        char_set.insert(c1);
        char_set.insert(c2);
        char_set.insert(c3);
        char_set.insert(c4);
        char_set.insert(c5);
        char_set.insert(c6);
        char_set.insert(c7);
        char_set.insert(c8);
        char_set.insert(c9);
        char_set.insert(c10);
        char_set.insert(c11);
        char_set.insert(c12);
        char_set.insert(c13);
        char_set.insert(c14);
        if char_set.len() == 14 {
            return end_pos;
        }
    }
    panic!("start of packet marker not found");
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    // input is a single line
    let position = find_start_of_packet_marker(&file_contents);
    println!("position: {}", position);
    position
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    // input is a single line
    let position = find_start_of_message_marker(&file_contents);
    println!("position: {}", position);
    position
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day6, part1, example1, 7);
    test_fn!(day6, part1, example2, 5);
    test_fn!(day6, part1, example3, 6);
    test_fn!(day6, part1, example4, 10);
    test_fn!(day6, part1, example5, 11);

    test_fn!(day6, part1, input, 1896);

    test_fn!(day6, part2, example1, 19);
    test_fn!(day6, part2, example2, 23);
    test_fn!(day6, part2, example3, 23);
    test_fn!(day6, part2, example4, 29);
    test_fn!(day6, part2, example5, 26);

    test_fn!(day6, part2, input, 3452);
}
