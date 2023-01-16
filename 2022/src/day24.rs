use std::collections::{HashSet, VecDeque};

use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;

use run_aoc::runner_fn;
use utils::simple_struct;

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    map(one_of("#.<^>v"), |c| match c {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        '<' => Tile::Left,
        '^' => Tile::Up,
        '>' => Tile::Right,
        'v' => Tile::Down,
        _ => unreachable!(),
    })(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(parse_tile)(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    terminated(separated_list1(newline, parse_line), newline)(input)
}

fn parse_input(input: &str) -> Vec<Vec<Tile>> {
    let (leftover, tiles) = parse_lines(input).expect("could not parse input");
    assert_eq!(leftover, "");
    tiles
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Left,
    Up,
    Right,
    Down,
}

simple_struct!([Copy] Pos; row: usize, col: usize);

struct Valley {
    start_pos: Pos,
    goal_pos: Pos,
    walls: HashSet<Pos>,
    blizzard: Blizzard,
    blizzard_limits: (Pos, Pos),
    map_limit: Pos,
}

impl Valley {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        let mut blizzard_tiles: Vec<Vec<Tile>> = Vec::new();
        let mut walls: HashSet<Pos> = HashSet::new();
        let mut start_pos = Pos::new(0, 0);
        let mut goal_pos = Pos::new(0, 0);

        for (row, tile_vec) in tiles.iter().enumerate() {
            if row == 0 {
                for (col, tile) in tile_vec.iter().enumerate() {
                    match tile {
                        Tile::Wall => {
                            walls.insert(Pos::new(row, col));
                        }
                        Tile::Empty => {
                            start_pos = Pos::new(row, col);
                        }
                        _ => unreachable!(),
                    }
                }
            } else if row == tiles.len() - 1 {
                for (col, tile) in tile_vec.iter().enumerate() {
                    match tile {
                        Tile::Wall => {
                            walls.insert(Pos::new(row, col));
                        }
                        Tile::Empty => {
                            goal_pos = Pos::new(row, col);
                        }
                        _ => unreachable!(),
                    }
                }
            } else {
                let mut blizzard_row: Vec<Tile> = Vec::new();
                for (col, tile) in tile_vec.iter().enumerate() {
                    match tile {
                        Tile::Wall => {
                            walls.insert(Pos::new(row, col));
                        }
                        _ => {
                            blizzard_row.push(*tile);
                        }
                    }
                }
                blizzard_tiles.push(blizzard_row);
            }
        }

        Valley {
            start_pos,
            goal_pos,
            walls,
            blizzard: Blizzard::from(blizzard_tiles),
            blizzard_limits: (
                Pos::new(1, 1),
                Pos::new(tiles.len() - 2, tiles[0].len() - 2),
            ),
            map_limit: Pos::new(tiles.len() - 1, tiles[0].len() - 1),
        }
    }

    fn time_to_goal(&mut self) -> usize {
        let mut possibilities: HashSet<Pos> = HashSet::new();
        possibilities.insert(self.start_pos);
        let mut num_steps = 0;

        'outer: loop {
            let mut new_possibilities: HashSet<Pos> = HashSet::new();
            self.blizzard.advance();

            for pos in possibilities.into_iter() {
                if pos == self.goal_pos {
                    // found it
                    break 'outer;
                }
                for new_move in self.possible_moves(&pos).into_iter() {
                    new_possibilities.insert(new_move);
                }
            }

            println!(
                "{} steps, {} possibilities",
                num_steps,
                new_possibilities.len()
            );
            if new_possibilities.len() == 0 {
                panic!("no more possible moves!");
            }
            possibilities = new_possibilities;
            num_steps += 1;
        }
        num_steps
    }

    fn time_to_goal_start_goal(&mut self) -> usize {
        let mut possibilities: HashSet<Pos> = HashSet::new();
        possibilities.insert(self.start_pos);
        let mut num_steps = 0;
        let mut first_goal = false;
        let mut back_to_start = false;

        'outer: loop {
            let mut new_possibilities: HashSet<Pos> = HashSet::new();
            self.blizzard.advance();

            for pos in possibilities.into_iter() {
                if pos == self.goal_pos {
                    if first_goal && back_to_start {
                        // found it, again
                        break 'outer;
                    }
                    if !first_goal {
                        // got to the goal the first time
                        println!("found goal x1");
                        first_goal = true;
                        new_possibilities = HashSet::new();
                        new_possibilities.insert(self.goal_pos);
                        break;
                    }
                }
                if pos == self.start_pos {
                    if first_goal && !back_to_start {
                        println!("got back to the start");
                        back_to_start = true;
                        new_possibilities = HashSet::new();
                        new_possibilities.insert(self.start_pos);
                        break;
                    }
                }
                for new_move in self.possible_moves(&pos).into_iter() {
                    new_possibilities.insert(new_move);
                }
            }

            println!(
                "{} steps, {} possibilities",
                num_steps,
                new_possibilities.len()
            );
            if new_possibilities.len() == 0 {
                panic!("no more possible moves!");
            }
            possibilities = new_possibilities;
            num_steps += 1;
        }
        num_steps
    }

    fn possible_moves(&self, pos: &Pos) -> Vec<Pos> {
        let mut possible: Vec<Pos> = Vec::new();
        // left
        if pos.col > 0 {
            possible.push(Pos::new(pos.row, pos.col - 1));
        }
        // up
        if pos.row > 0 {
            possible.push(Pos::new(pos.row - 1, pos.col));
        }
        // right
        if pos.col < self.map_limit.col {
            possible.push(Pos::new(pos.row, pos.col + 1));
        }
        // down
        if pos.row < self.map_limit.row {
            possible.push(Pos::new(pos.row + 1, pos.col));
        }
        // wait
        possible.push(Pos::new(pos.row, pos.col));

        possible
            .into_iter()
            .filter(|p| {
                if self.in_blizzard(p) {
                    self.blizzard.is_empty(Pos::new(p.row - 1, p.col - 1))
                } else {
                    self.walls.get(p).is_none()
                }
            })
            .collect()
    }

    fn in_blizzard(&self, pos: &Pos) -> bool {
        self.blizzard_limits.0.row <= pos.row
            && pos.row <= self.blizzard_limits.1.row
            && self.blizzard_limits.0.col <= pos.col
            && pos.col <= self.blizzard_limits.1.col
    }
}

struct Blizzard {
    // use VecDeque to make rotation nicer (maybe faster?)
    left: Vec<VecDeque<bool>>,
    up: VecDeque<Vec<bool>>,
    right: Vec<VecDeque<bool>>,
    down: VecDeque<Vec<bool>>,
}

impl Blizzard {
    // only the tiles of the blizzard stuff, no walls
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        let left: Vec<VecDeque<bool>> = tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Tile::Left => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();
        let up: VecDeque<Vec<bool>> = tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Tile::Up => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();
        let right: Vec<VecDeque<bool>> = tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Tile::Right => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();
        let down: VecDeque<Vec<bool>> = tiles
            .iter()
            .map(|row| {
                row.iter()
                    .map(|t| match t {
                        Tile::Down => true,
                        _ => false,
                    })
                    .collect()
            })
            .collect();

        Blizzard {
            left,
            up,
            right,
            down,
        }
    }

    fn advance(&mut self) {
        // rotate each thing in the appropriate direction (with wraparound)
        for row in self.left.iter_mut() {
            let tile = row.pop_front().unwrap();
            row.push_back(tile);
        }
        for row in self.right.iter_mut() {
            let tile = row.pop_back().unwrap();
            row.push_front(tile);
        }
        let up_row = self.up.pop_front().unwrap();
        self.up.push_back(up_row);
        let down_row = self.down.pop_back().unwrap();
        self.down.push_front(down_row);
    }

    fn is_empty(&self, pos: Pos) -> bool {
        self.left[pos.row].get(pos.col).unwrap() == &false
            && self.up.get(pos.row).unwrap()[pos.col] == false
            && self.right[pos.row].get(pos.col).unwrap() == &false
            && self.down.get(pos.row).unwrap()[pos.col] == false
    }

    // for debugging
    fn _print(&self) {
        println!("left:");
        for row in self.left.iter() {
            println!("{:?}", row);
        }
        println!("up:");
        for row in self.up.iter() {
            println!("{:?}", row);
        }
        println!("right:");
        for row in self.right.iter() {
            println!("{:?}", row);
        }
        println!("down:");
        for row in self.down.iter() {
            println!("{:?}", row);
        }
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    let mut valley = Valley::from(parse_input(&file_contents));
    valley.time_to_goal()
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let mut valley = Valley::from(parse_input(&file_contents));
    valley.time_to_goal_start_goal()
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day24, part1, example, 18);
    test_fn!(day24, part1, input, 245);

    test_fn!(day24, part2, example, 54);
    test_fn!(day24, part2_SLOW, input, 798);
}
