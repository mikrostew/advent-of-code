use run_aoc::runner_fn;

fn snafu_to_dec(snafu: &str) -> isize {
    let mut power_of_5 = 1;
    let mut result = 0;
    for c in snafu.chars().rev() {
        let multiple = match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!(),
        };
        result += multiple * power_of_5;
        power_of_5 *= 5;
    }
    result
}

fn dec_to_snafu(dec: isize) -> String {
    let mut chars: Vec<char> = Vec::new();
    // repeated division by 5, but everything is shifted by 2 because ugh
    let mut quot = dec;
    while quot > 0 {
        let rem = ((quot + 2) % 5) - 2;
        chars.push(match rem {
            2 => '2',
            1 => '1',
            0 => '0',
            -1 => '-',
            -2 => '=',
            _ => unreachable!(),
        });
        quot = (quot + 2) / 5;
    }

    chars.into_iter().rev().collect()
}

#[runner_fn]
fn part1(file_contents: String) -> String {
    let sum = file_contents.lines().map(snafu_to_dec).sum::<isize>();
    dec_to_snafu(sum)
}

// no part2?
#[runner_fn]
fn part2(file_contents: String) -> usize {
    println!("{}", file_contents);
    0
}

#[cfg(test)]
mod tests {
    use super::{dec_to_snafu, snafu_to_dec};
    use run_aoc::test_fn;

    #[test]
    fn test_snafu_to_dec() {
        assert_eq!(snafu_to_dec("1=-0-2"), 1747);
        assert_eq!(snafu_to_dec("12111"), 906);
        assert_eq!(snafu_to_dec("2=0="), 198);
        assert_eq!(snafu_to_dec("21"), 11);
        assert_eq!(snafu_to_dec("2=01"), 201);
        assert_eq!(snafu_to_dec("111"), 31);
        assert_eq!(snafu_to_dec("20012"), 1257);
        assert_eq!(snafu_to_dec("112"), 32);
        assert_eq!(snafu_to_dec("1=-1="), 353);
        assert_eq!(snafu_to_dec("1-12"), 107);
        assert_eq!(snafu_to_dec("12"), 7);
        assert_eq!(snafu_to_dec("1="), 3);
        assert_eq!(snafu_to_dec("122"), 37);
    }

    #[test]
    fn test_dec_to_snafu() {
        assert_eq!(dec_to_snafu(1), "1");
        assert_eq!(dec_to_snafu(2), "2");
        assert_eq!(dec_to_snafu(3), "1=");
        assert_eq!(dec_to_snafu(4), "1-");
        assert_eq!(dec_to_snafu(5), "10");
        assert_eq!(dec_to_snafu(6), "11");
        assert_eq!(dec_to_snafu(7), "12");
        assert_eq!(dec_to_snafu(8), "2=");
        assert_eq!(dec_to_snafu(9), "2-");
        assert_eq!(dec_to_snafu(10), "20");
        assert_eq!(dec_to_snafu(15), "1=0");
        assert_eq!(dec_to_snafu(20), "1-0");
        assert_eq!(dec_to_snafu(2022), "1=11-2");
        assert_eq!(dec_to_snafu(4890), "2=-1=0");
        assert_eq!(dec_to_snafu(12345), "1-0---0");
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
    }

    test_fn!(day25, part1, example, "2=-1=0");
    test_fn!(day25, part1, input, "2=-0=01----22-0-1-10");

    // no part2?
    // test_fn!(day25, part2, example, 0);
    // test_fn!(day25, part2, input, 0);
}
