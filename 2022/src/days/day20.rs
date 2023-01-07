use run_aoc::runner_fn;

use super::expect_isize;

fn parse_input(input: &str, key: isize) -> Vec<isize> {
    input.lines().map(|l| expect_isize!(l) * key).collect()
}

struct Decrypt {
    original: Vec<isize>,
    // input num and it's original index (b/c there are dups in the full input)
    decrypted: Vec<(usize, isize)>,
}

impl Decrypt {
    fn from(original: Vec<isize>) -> Self {
        let decrypted: Vec<(usize, isize)> =
            original.iter().enumerate().map(|(i, v)| (i, *v)).collect();
        Decrypt {
            original,
            decrypted,
        }
    }

    fn mix_times(&mut self, n: usize) {
        for _ in 0..n {
            self.mix();
        }
    }

    fn mix(&mut self) {
        for (i, &val) in self.original.iter().enumerate() {
            let start_index = self
                .decrypted
                .iter()
                .position(|(a, b)| *a == i && *b == val)
                .unwrap();
            let end_index = self.calc_end_index(start_index, val);
            // using rotate_* is on the slice is maybe slow because it takes O(n) time, whatever,
            // fast enough ¯\_(ツ)_/¯
            if start_index < end_index {
                self.decrypted[start_index..=end_index].rotate_left(1);
            } else {
                self.decrypted[end_index..=start_index].rotate_right(1);
            }
        }
    }

    fn calc_end_index(&self, start: usize, diff: isize) -> usize {
        // ugh, this off-by-one took too long to figure out,
        // this is one less than the size of _all_ the numbers, b/c it's moving the target num
        // thru all the _other_ nums (not including itself)
        let len_other_nums = (self.decrypted.len() as isize) - 1;
        let mut end_index = (start as isize + diff) % len_other_nums;
        if end_index < 0 {
            end_index += len_other_nums;
        }
        end_index as usize
    }

    fn sum_of_coords(&self) -> isize {
        let zero_index = self.decrypted.iter().position(|(_, x)| *x == 0).unwrap();
        println!("index of 0 is {}", zero_index);
        let coord1 = self.decrypted[self.wrap_index(zero_index + 1000)].1;
        let coord2 = self.decrypted[self.wrap_index(zero_index + 2000)].1;
        let coord3 = self.decrypted[self.wrap_index(zero_index + 3000)].1;
        println!("{}, {}, {}", coord1, coord2, coord3);
        coord1 + coord2 + coord3
    }

    fn wrap_index(&self, index: usize) -> usize {
        index % self.decrypted.len()
    }
}

#[runner_fn]
fn part1(file_contents: String) -> isize {
    let mut decrypt = Decrypt::from(parse_input(&file_contents, 1));
    decrypt.mix();
    decrypt.sum_of_coords()
}

#[runner_fn]
fn part2(file_contents: String) -> isize {
    let mut decrypt = Decrypt::from(parse_input(&file_contents, 811589153));
    decrypt.mix_times(10);
    decrypt.sum_of_coords()
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day20, part1, example, 3);
    test_fn!(day20, part1, input, 3700);

    test_fn!(day20, part2, example, 1623178306);
    // TODO: too slow to run this all the time
    //test_fn!(day20, part2, input, 10626948369382);
}
