use std::collections::HashSet;

use super::run_parts;

run_parts!();

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

fn part1(file_contents: String) -> String {
    // input is a single line
    let position = find_start_of_packet_marker(&file_contents);
    println!("position: {}", position);
    format!("{}", position)
}

fn part2(file_contents: String) -> String {
    // input is a single line
    let position = find_start_of_message_marker(&file_contents);
    println!("position: {}", position);
    format!("{}", position)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example1() {
        let input = read_input_file("inputs/day6-example1.txt");
        assert_eq!(part1(input), "7".to_string());
    }

    #[test]
    fn part1_example2() {
        let input = read_input_file("inputs/day6-example2.txt");
        assert_eq!(part1(input), "5".to_string());
    }

    #[test]
    fn part1_example3() {
        let input = read_input_file("inputs/day6-example3.txt");
        assert_eq!(part1(input), "6".to_string());
    }

    #[test]
    fn part1_example4() {
        let input = read_input_file("inputs/day6-example4.txt");
        assert_eq!(part1(input), "10".to_string());
    }

    #[test]
    fn part1_example5() {
        let input = read_input_file("inputs/day6-example5.txt");
        assert_eq!(part1(input), "11".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day6-input.txt");
        assert_eq!(part1(input), "1896".to_string());
    }

    #[test]
    fn part2_example1() {
        let input = read_input_file("inputs/day6-example1.txt");
        assert_eq!(part2(input), "19".to_string());
    }

    #[test]
    fn part2_example2() {
        let input = read_input_file("inputs/day6-example2.txt");
        assert_eq!(part2(input), "23".to_string());
    }

    #[test]
    fn part2_example3() {
        let input = read_input_file("inputs/day6-example3.txt");
        assert_eq!(part2(input), "23".to_string());
    }

    #[test]
    fn part2_example4() {
        let input = read_input_file("inputs/day6-example4.txt");
        assert_eq!(part2(input), "29".to_string());
    }

    #[test]
    fn part2_example5() {
        let input = read_input_file("inputs/day6-example5.txt");
        assert_eq!(part2(input), "26".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day6-input.txt");
        assert_eq!(part2(input), "3452".to_string());
    }
}
