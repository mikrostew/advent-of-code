use std::collections::{HashMap, HashSet};

use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;
use run_aoc::runner_fn;

use utils::simple_struct;

fn parse_line(input: &str) -> IResult<&str, Vec<bool>> {
    many1(map(one_of(".#"), |c| match c {
        '.' => false,
        '#' => true,
        _ => unreachable!(),
    }))(input)
}

fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<bool>>> {
    terminated(separated_list1(newline, parse_line), newline)(input)
}

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    let (leftover, grid) = parse_lines(input).expect("could not parse input");
    assert_eq!(leftover, "");
    grid
}

simple_struct!([Copy] Point; row: isize, col: isize);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum DirList {
    N,
    S,
    E,
    W,
}

impl DirList {
    fn iterator(for_round: usize) -> impl Iterator<Item = DirList> {
        match for_round % 4 {
            1 => [DirList::N, DirList::S, DirList::W, DirList::E]
                .iter()
                .copied(),
            2 => [DirList::S, DirList::W, DirList::E, DirList::N]
                .iter()
                .copied(),
            3 => [DirList::W, DirList::E, DirList::N, DirList::S]
                .iter()
                .copied(),
            0 => [DirList::E, DirList::N, DirList::S, DirList::W]
                .iter()
                .copied(),
            _ => unreachable!(),
        }
    }
}

struct ElfGrid {
    elves: HashSet<Point>,
    round: usize,
}

impl ElfGrid {
    fn from(in_grid: Vec<Vec<bool>>) -> Self {
        let mut elves: HashSet<Point> = HashSet::new();

        for row in 0..in_grid.len() {
            for (col, elem) in in_grid[row].iter().enumerate() {
                if *elem {
                    elves.insert(Point::new(row.try_into().unwrap(), col.try_into().unwrap()));
                }
            }
        }
        ElfGrid { elves, round: 0 }
    }

    fn do_rounds(&mut self, num_rounds: usize) {
        // println!("start state:");
        // self._print_grid();
        for _ in 0..num_rounds {
            self.do_round();
            // println!("round {}:", self.round);
            // self._print_grid();
        }
    }

    // return T/F if an elf did a move this round
    fn do_round(&mut self) -> bool {
        self.round += 1;
        let mut did_move = false;
        // map<to, from>
        let mut proposed_moves: HashMap<Point, Point> = HashMap::new();

        for elf_pos in self.elves.iter() {
            if let Some(new_pos) = self.should_move(elf_pos) {
                if proposed_moves.get(&new_pos).is_some() {
                    // use this value to filter out moves
                    proposed_moves.insert(new_pos, Point::new(isize::MAX, isize::MIN));
                } else {
                    proposed_moves.insert(new_pos, *elf_pos);
                }
            }
        }

        for (to_pos, from_pos) in proposed_moves.iter() {
            if from_pos != &Point::new(isize::MAX, isize::MIN) {
                self.elves.remove(from_pos);
                self.elves.insert(*to_pos);
                did_move = true;
            }
        }
        did_move
    }

    fn should_move(&self, from_pos: &Point) -> Option<Point> {
        let (n, ne, e, se, s, sw, w, nw) = (
            self.elves.get(&Point::new(from_pos.row - 1, from_pos.col)),
            self.elves
                .get(&Point::new(from_pos.row - 1, from_pos.col + 1)),
            self.elves.get(&Point::new(from_pos.row, from_pos.col + 1)),
            self.elves
                .get(&Point::new(from_pos.row + 1, from_pos.col + 1)),
            self.elves.get(&Point::new(from_pos.row + 1, from_pos.col)),
            self.elves
                .get(&Point::new(from_pos.row + 1, from_pos.col - 1)),
            self.elves.get(&Point::new(from_pos.row, from_pos.col - 1)),
            self.elves
                .get(&Point::new(from_pos.row - 1, from_pos.col - 1)),
        );
        if (n, ne, e, se, s, sw, w, nw) == (None, None, None, None, None, None, None, None) {
            return None;
        }
        for d in DirList::iterator(self.round) {
            match d {
                DirList::N => {
                    if (nw, n, ne) == (None, None, None) {
                        return Some(Point::new(from_pos.row - 1, from_pos.col));
                    }
                }
                DirList::S => {
                    if (sw, s, se) == (None, None, None) {
                        return Some(Point::new(from_pos.row + 1, from_pos.col));
                    }
                }
                DirList::E => {
                    if (ne, e, se) == (None, None, None) {
                        return Some(Point::new(from_pos.row, from_pos.col + 1));
                    }
                }
                DirList::W => {
                    if (nw, w, sw) == (None, None, None) {
                        return Some(Point::new(from_pos.row, from_pos.col - 1));
                    }
                }
            }
        }
        // nowhere to move
        None
    }

    fn containing_rect(&self) -> (isize, isize, isize, isize) {
        let (mut row_min, mut row_max, mut col_min, mut col_max) =
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN);
        for elf_pos in self.elves.iter() {
            row_min = row_min.min(elf_pos.row);
            row_max = row_max.max(elf_pos.row);
            col_min = col_min.min(elf_pos.col);
            col_max = col_max.max(elf_pos.col);
        }
        (row_min, row_max, col_min, col_max)
    }

    fn count_empty_tiles(&self) -> usize {
        let (row_min, row_max, col_min, col_max) = self.containing_rect();
        let mut empty_tiles = 0;
        for r in row_min..=row_max {
            for c in col_min..=col_max {
                if self.elves.get(&Point::new(r, c)).is_none() {
                    empty_tiles += 1;
                }
            }
        }
        empty_tiles
    }

    fn disperse(&mut self) -> usize {
        while self.do_round() {}
        self.round
    }

    // for debugging this mess
    fn _print_grid(&self) {
        let (row_min, row_max, col_min, col_max) = self.containing_rect();
        for r in row_min..=row_max {
            for c in col_min..=col_max {
                match self.elves.get(&Point::new(r, c)) {
                    Some(_) => print!("#"),
                    None => print!("."),
                }
            }
            println!("");
        }
        println!("");
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    let mut elf_grid = ElfGrid::from(parse_input(&file_contents));
    elf_grid.do_rounds(10);
    elf_grid.count_empty_tiles()
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let mut elf_grid = ElfGrid::from(parse_input(&file_contents));
    elf_grid.disperse()
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day23, part1, example, 110);
    test_fn!(day23, part1, input, 3966);

    test_fn!(day23, part2, example, 20);
    // TODO: this takes too long to run consistently
    //test_fn!(day23, part2, input, 933);
}
