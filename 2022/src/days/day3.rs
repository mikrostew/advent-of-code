use std::collections::HashSet;
use std::path::Path;

use super::{read_file, run_parts};

run_parts!();

fn find_item_priority(line: &str) -> u32 {
    let line_len = line.len();
    if line_len % 2 != 0 {
        panic!("Line length {} is not even - cannot split it", line_len);
    }
    let half_line_len = line_len / 2;
    //println!("Line: {}", line);
    let common_char = find_common_char(&line[0..half_line_len], &line[half_line_len..]);
    //println!("common char: {}", common_char);
    priority_for_char(common_char)
}

fn find_common_char(first_half: &str, second_half: &str) -> char {
    let mut char_set = HashSet::new();
    let mut common_chars = HashSet::new();

    first_half.chars().for_each(|c| {
        char_set.insert(c);
    });
    second_half.chars().for_each(|c| {
        if char_set.contains(&c) {
            common_chars.insert(c);
        }
    });
    let common_char: Vec<&char> = common_chars.iter().collect();
    if common_char.len() != 1 {
        panic!("Found {} common chars, expected 1", common_char.len());
    }
    *common_char[0]
}

fn find_common_char_3_lines(line1: &str, line2: &str, line3: &str) -> char {
    let mut char_set = HashSet::new();
    let mut common_chars_2_lines = HashSet::new();
    let mut common_chars_3_lines = HashSet::new();

    // set of all chars in the first line
    line1.chars().for_each(|c| {
        char_set.insert(c);
    });
    // find common chars with the second line
    line2.chars().for_each(|c| {
        if char_set.contains(&c) {
            common_chars_2_lines.insert(c);
        }
    });
    // find common chars with the third line
    line3.chars().for_each(|c| {
        if common_chars_2_lines.contains(&c) {
            common_chars_3_lines.insert(c);
        }
    });

    let common_char: Vec<&char> = common_chars_3_lines.iter().collect();
    if common_char.len() != 1 {
        panic!("Found {} common chars, expected 1", common_char.len());
    }
    *common_char[0]
}

fn priority_for_char(c: char) -> u32 {
    match c {
        // a-z is 1-26, (a is 97 in ascii)
        'a'..='z' => (c as u32) - 96,
        // A-Z is 27-52, (A is 65 in ascii)
        'A'..='Z' => (c as u32) - 38,
        _ => panic!("Not a letter! {}", c),
    }
}

fn part1<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);

    let item_priorities: Vec<u32> = file_contents
        .lines()
        .map(|line| find_item_priority(line))
        .collect();

    println!("Item priorities: {:?}", item_priorities);
    println!("Total priority: {}", item_priorities.iter().sum::<u32>());
}

fn part2<P: AsRef<Path>>(path: P) -> () {
    read_file!(file_contents, path);

    // process this in groups of 3
    let badge_priorities: Vec<u32> = file_contents
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            if chunk.len() != 3 {
                panic!("these groups are not divisible by 3");
            }
            let common_char = find_common_char_3_lines(chunk[0], chunk[1], chunk[2]);
            println!("common char: {}", common_char);
            priority_for_char(common_char)
        })
        .collect();

    println!("Badge priorities: {:?}", badge_priorities);
    println!("Total priority: {}", badge_priorities.iter().sum::<u32>());
}
