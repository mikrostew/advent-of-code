use std::collections::HashSet;

use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::one_of;
use nom::sequence::separated_pair;
use nom::IResult;

use super::expect_usize;
use super::simple_struct;
use run_aoc::runner_fn;

#[derive(Debug)]
enum Direction {
    Left(usize),
    Up(usize),
    Right(usize),
    Down(usize),
}

fn parse_line(input: &str) -> IResult<&str, Direction> {
    direction(input)
}

fn direction(input: &str) -> IResult<&str, Direction> {
    separated_pair(one_of("LURD"), tag(" "), distance)(input).map(|(next_input, (dir, dist))| {
        (
            next_input,
            match dir {
                'L' => Direction::Left(dist),
                'U' => Direction::Up(dist),
                'R' => Direction::Right(dist),
                'D' => Direction::Down(dist),
                _ => panic!("Matched something that was not L-U-R-D!! how?!"),
            },
        )
    })
}

fn distance(input: &str) -> IResult<&str, usize> {
    digit1(input).map(|(next_input, d)| (next_input, expect_usize!(d)))
}

simple_struct!(Point; x: i32, y: i32);

struct RopeBridge2Knots {
    head_pos: Point,
    tail_pos: Point,
    tail_visited: HashSet<String>,
}

impl RopeBridge2Knots {
    fn new() -> Self {
        RopeBridge2Knots {
            // start out at the same point
            head_pos: Point::new(0, 0),
            tail_pos: Point::new(0, 0),
            tail_visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, d: &Direction) -> () {
        match d {
            Direction::Left(dist) => {
                for _ in 0..*dist {
                    self.head_pos.x -= 1;
                    self.update_tail_pos();
                }
            }
            Direction::Up(dist) => {
                for _ in 0..*dist {
                    self.head_pos.y += 1;
                    self.update_tail_pos();
                }
            }
            Direction::Right(dist) => {
                for _ in 0..*dist {
                    self.head_pos.x += 1;
                    self.update_tail_pos();
                }
            }
            Direction::Down(dist) => {
                for _ in 0..*dist {
                    self.head_pos.y -= 1;
                    self.update_tail_pos();
                }
            }
        }
    }

    fn update_tail_pos(&mut self) -> () {
        if (self.head_pos.x - self.tail_pos.x).abs() > 1 {
            // is this a diagonal move?
            if (self.head_pos.y - self.tail_pos.y).abs() == 1 {
                // adjust this first, for a diagonal move
                self.tail_pos.y = self.head_pos.y;
            }
            // figure out which way to go
            if self.tail_pos.x < self.head_pos.x {
                self.tail_pos.x += 1;
            } else {
                self.tail_pos.x -= 1;
            }
        } else if (self.head_pos.y - self.tail_pos.y).abs() > 1 {
            // is this a diagonal move?
            if (self.head_pos.x - self.tail_pos.x).abs() == 1 {
                // adjust this first, for a diagonal move
                self.tail_pos.x = self.head_pos.x;
            }
            // figure out which way to go
            if self.tail_pos.y < self.head_pos.y {
                self.tail_pos.y += 1;
            } else {
                self.tail_pos.y -= 1;
            }
        }
        self.tail_visited
            .insert(format!("{},{}", self.tail_pos.x, self.tail_pos.y));
    }

    fn num_tail_positions(&self) -> usize {
        self.tail_visited.len()
    }
}

struct RopeBridge10Knots {
    knots: Vec<Point>,
    tail_visited: HashSet<String>,
}

impl RopeBridge10Knots {
    fn new() -> Self {
        // everything starts at 0,0
        let mut knots: Vec<Point> = vec![];
        for _ in 0..10 {
            knots.push(Point::new(0, 0));
        }
        RopeBridge10Knots {
            knots,
            tail_visited: HashSet::new(),
        }
    }

    fn move_head(&mut self, d: &Direction) -> () {
        match d {
            Direction::Left(dist) => {
                for _ in 0..*dist {
                    self.knots[0].x -= 1;
                    self.update_rest_of_knots();
                }
            }
            Direction::Up(dist) => {
                for _ in 0..*dist {
                    self.knots[0].y += 1;
                    self.update_rest_of_knots();
                }
            }
            Direction::Right(dist) => {
                for _ in 0..*dist {
                    self.knots[0].x += 1;
                    self.update_rest_of_knots();
                }
            }
            Direction::Down(dist) => {
                for _ in 0..*dist {
                    self.knots[0].y -= 1;
                    self.update_rest_of_knots();
                }
            }
        }
    }

    fn update_rest_of_knots(&mut self) -> () {
        // loop over all knots, each pair is like head/tail from before
        // BUT, the distance between knots can be larger this time
        // (I did not account for that the first time, oops)
        for h in 0..9 {
            let t = h + 1;
            let delta_x = self.knots[h].x - self.knots[t].x;
            let delta_y = self.knots[h].y - self.knots[t].y;
            //println!("{}: delta_x: {}, delta_y: {}", t, delta_x, delta_y);

            if delta_x.abs() == 2 {
                self.knots[t].x += delta_x / 2;
                if delta_y.abs() == 1 {
                    self.knots[t].y += delta_y;
                } else if delta_y.abs() == 2 {
                    self.knots[t].y += delta_y / 2;
                }
            } else if delta_y.abs() == 2 {
                self.knots[t].y += delta_y / 2;
                if delta_x.abs() == 1 {
                    self.knots[t].x += delta_x;
                } else if delta_x.abs() == 2 {
                    self.knots[t].x += delta_x / 2;
                }
            }
        }
        self.tail_visited
            .insert(format!("{},{}", self.knots[9].x, self.knots[9].y));
    }

    // debugging part2 failure
    fn _print_positions(&self) -> () {
        let mut pos: Vec<Vec<char>> = vec![];
        for _ in 0..10 {
            let mut row: Vec<char> = vec![];
            for _ in 0..10 {
                row.push('.');
            }
            pos.push(row);
        }
        for n in 0..10 {
            pos[self.knots[n].y as usize][self.knots[n].x as usize] = match n {
                0 => 'H',
                1 => '1',
                2 => '2',
                3 => '3',
                4 => '4',
                5 => '5',
                6 => '6',
                7 => '7',
                8 => '8',
                9 => '9',
                _ => panic!("there are only 10 knots, shouldn't get here"),
            };
        }
        for row in 0..10 {
            println!("{:?}", pos[row]);
        }
        println!("");
    }

    fn num_tail_positions(&self) -> usize {
        self.tail_visited.len()
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    let directions: Vec<Direction> = file_contents
        .lines()
        .map(|l| {
            let (leftover, parsed) = parse_line(l).expect("Could not parse line!");
            assert_eq!(leftover, "");
            parsed
        })
        .collect();

    let mut bridge = RopeBridge2Knots::new();
    directions.iter().for_each(|d| bridge.move_head(d));

    bridge.num_tail_positions()
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let directions: Vec<Direction> = file_contents
        .lines()
        .map(|l| {
            let (leftover, parsed) = parse_line(l).expect("Could not parse line!");
            assert_eq!(leftover, "");
            parsed
        })
        .collect();

    let mut bridge = RopeBridge10Knots::new();
    directions.iter().for_each(|d| bridge.move_head(d));

    bridge.num_tail_positions()
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day9, part1, example, 13);
    test_fn!(day9, part1, input, 5683);

    test_fn!(day9, part2, example, 1);
    test_fn!(day9, part2, example2, 36);

    test_fn!(day9, part2, input, 2372);
}
