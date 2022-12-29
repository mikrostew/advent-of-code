use nom::bytes::complete::take;
use nom::multi::many1;
use nom::IResult;

use super::expect_i32;
use run_aoc::runner_fn;

// lines are just a bunch a digits
fn parse_line(input: &str) -> IResult<&str, Vec<i32>> {
    many1(digit)(input)
}

// parse a single digit as u8
fn digit(input: &str) -> IResult<&str, i32> {
    take(1usize)(input).map(|(next_input, d)| (next_input, expect_i32!(d)))
}

struct TreeGrid {
    grid: Vec<Vec<i32>>,
    num_rows: usize,
    num_cols: usize,
    tallest_from_left: Vec<Vec<i32>>,
    tallest_from_top: Vec<Vec<i32>>,
    tallest_from_right: Vec<Vec<i32>>,
    tallest_from_bottom: Vec<Vec<i32>>,
}

impl TreeGrid {
    fn new(grid: Vec<Vec<i32>>) -> Self {
        // could validate rows are all the same length, but whatever
        let num_rows = grid.len();
        let num_cols = grid[0].len();
        TreeGrid {
            grid,
            num_rows,
            num_cols,
            tallest_from_left: vec![],
            tallest_from_top: vec![],
            tallest_from_right: vec![],
            tallest_from_bottom: vec![],
        }
    }

    // pre-calculate the tallest trees, so this is faster
    fn calculate_tallest_trees(&mut self) -> () {
        // first do from left and from top
        for row in 0..self.num_rows {
            let mut from_left_vec = vec![];
            let mut from_top_vec = vec![];
            for col in 0..self.num_cols {
                // from left
                // things on the outsides have nothing next to them
                if col == 0 {
                    from_left_vec.push(-1);
                } else {
                    // figure out the tallest tree seen so far
                    from_left_vec.push(if self.grid[row][col - 1] > from_left_vec[col - 1] {
                        self.grid[row][col - 1]
                    } else {
                        from_left_vec[col - 1]
                    });
                }
                // from top
                // things on the outsides have nothing next to them
                if row == 0 {
                    from_top_vec.push(-1);
                } else {
                    // figure out the tallest tree so far
                    from_top_vec.push(
                        if self.grid[row - 1][col] > self.tallest_from_top[row - 1][col] {
                            self.grid[row - 1][col]
                        } else {
                            self.tallest_from_top[row - 1][col]
                        },
                    );
                }
            }
            self.tallest_from_left.push(from_left_vec);
            self.tallest_from_top.push(from_top_vec);
        }
        // then do from right and from bottom
        for row in (0..self.num_rows).rev() {
            let mut from_right_vec = vec![];
            let mut from_bottom_vec = vec![];
            for col in (0..self.num_cols).rev() {
                // from right
                // things on the outsides have nothing next to them
                if col == self.num_cols - 1 {
                    from_right_vec.push(-1);
                } else {
                    // figure out the tallest tree seen so far
                    // (doing this reversed)
                    let reversed_index = self.num_cols - col - 2;
                    from_right_vec.push(
                        if self.grid[row][col + 1] > from_right_vec[reversed_index] {
                            self.grid[row][col + 1]
                        } else {
                            from_right_vec[reversed_index]
                        },
                    );
                }
                // from bottom
                // things on the outsides have nothing next to them
                if row == self.num_rows - 1 {
                    from_bottom_vec.push(-1);
                } else {
                    // figure out the tallest tree so far
                    // (doing this reversed)
                    let reversed_index = self.num_rows - row - 2;
                    from_bottom_vec.push(
                        if self.grid[row + 1][col] > self.tallest_from_bottom[reversed_index][col] {
                            self.grid[row + 1][col]
                        } else {
                            self.tallest_from_bottom[reversed_index][col]
                        },
                    );
                }
            }
            from_right_vec.reverse();
            from_bottom_vec.reverse();
            self.tallest_from_right.push(from_right_vec);
            self.tallest_from_bottom.push(from_bottom_vec);
        }
        self.tallest_from_right.reverse();
        self.tallest_from_bottom.reverse();
    }

    fn count_visible_trees(&self) -> usize {
        let mut num_visible = 0;
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                let current = self.grid[row][col];
                if current > self.tallest_from_left[row][col]
                    || current > self.tallest_from_top[row][col]
                    || current > self.tallest_from_right[row][col]
                    || current > self.tallest_from_bottom[row][col]
                {
                    num_visible += 1;
                }
            }
        }
        num_visible
    }

    // I don't see a simple way to pre-calculate for this one
    fn calculate_scenic_scores(&self) -> Vec<i32> {
        let mut scores: Vec<i32> = vec![];
        for row in 0..self.num_rows {
            for col in 0..self.num_cols {
                // edges are 0
                if row == 0 || row == self.num_rows - 1 || col == 0 || col == self.num_cols - 1 {
                    scores.push(0);
                } else {
                    let current = self.grid[row][col];

                    let mut left = 0;
                    for temp_col in (0..col).rev() {
                        left += 1;
                        if self.grid[row][temp_col] >= current {
                            break;
                        }
                    }

                    let mut top = 0;
                    for temp_row in (0..row).rev() {
                        top += 1;
                        if self.grid[temp_row][col] >= current {
                            break;
                        }
                    }

                    let mut right = 0;
                    for temp_col in (col + 1)..self.num_cols {
                        right += 1;
                        if self.grid[row][temp_col] >= current {
                            break;
                        }
                    }

                    let mut bottom = 0;
                    for temp_row in (row + 1)..self.num_rows {
                        bottom += 1;
                        if self.grid[temp_row][col] >= current {
                            break;
                        }
                    }

                    scores.push(left * top * right * bottom);
                }
            }
        }
        scores
    }

    fn print(&self) -> () {
        print_grid(&self.grid, self.num_rows);
    }

    fn _print_tallest_trees(&self) -> () {
        println!("");
        println!("left, top, right, bottom");
        println!("");
        [
            &self.tallest_from_left,
            &self.tallest_from_top,
            &self.tallest_from_right,
            &self.tallest_from_bottom,
        ]
        .iter()
        .for_each(|g| {
            print_grid(g, self.num_rows);
            println!("");
        });
    }
}

fn print_grid(grid: &Vec<Vec<i32>>, num_rows: usize) -> () {
    for row in 0..num_rows {
        println!(
            "{}",
            grid[row]
                .iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    //println!("{}", file_contents);

    let grid: Vec<Vec<i32>> = file_contents
        .lines()
        .map(|l| {
            let (leftover, digits) = parse_line(l).expect("Could not parse line!");
            assert_eq!(leftover, "");
            digits
        })
        .collect();

    let mut tree_grid = TreeGrid::new(grid);
    //tree_grid.print();

    tree_grid.calculate_tallest_trees();
    //tree_grid.print_tallest_trees();

    tree_grid.count_visible_trees()
}

#[runner_fn]
fn part2(file_contents: String) -> i32 {
    let grid: Vec<Vec<i32>> = file_contents
        .lines()
        .map(|l| {
            let (leftover, digits) = parse_line(l).expect("Could not parse line!");
            assert_eq!(leftover, "");
            digits
        })
        .collect();

    let tree_grid = TreeGrid::new(grid);
    tree_grid.print();
    let scores = tree_grid.calculate_scenic_scores();
    println!("{:?}", scores);

    *scores.iter().max().expect("no max value?")
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day8, part1, example, 21);
    test_fn!(day8, part1, input, 1835);

    test_fn!(day8, part2, example, 8);
    test_fn!(day8, part2, input, 263670);
}
