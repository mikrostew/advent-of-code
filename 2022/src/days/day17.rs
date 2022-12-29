use std::collections::HashSet;

use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

use super::simple_struct;
use run_aoc::runner_fn;

simple_struct!(Point; x: usize, y: usize);

impl Point {
    fn try_move(&self, dir: Move) -> Option<Self> {
        match dir {
            Move::Left => {
                if self.x > 0 {
                    Some(Point::new(self.x - 1, self.y))
                } else {
                    None
                }
            }
            Move::Right => Some(Point::new(self.x + 1, self.y)),
            Move::Down => {
                if self.y > 0 {
                    Some(Point::new(self.x, self.y - 1))
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Move {
    // there is no upward movement
    Left,
    Right,
    Down,
}

struct Jets {
    jets: Vec<Move>,
    current: usize,
}

impl Jets {
    fn new(jets: Vec<Move>) -> Self {
        Jets { jets, current: 0 }
    }
}

impl Iterator for Jets {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        let jet = self.jets[self.current];
        self.current += 1;
        // if it gets to the end, repeat
        if self.current == self.jets.len() {
            self.current = 0;
        }
        Some(jet)
    }
}

fn parse_into_jets(input: &str) -> Jets {
    let (leftover, jets) = parse_jets(input).expect("Could not parse jets");
    assert_eq!(leftover, "");
    println!("Parsed {} jets", jets.len());
    Jets::new(jets)
}

fn parse_jets(input: &str) -> IResult<&str, Vec<Move>> {
    terminated(many1(jet), newline)(input)
}

fn jet(input: &str) -> IResult<&str, Move> {
    map(one_of("<>"), |c: char| match c {
        '<' => Move::Left,
        '>' => Move::Right,
        _ => panic!("Invalid jet char"),
    })(input)
}

#[derive(Clone, Copy, Debug)]
enum Rock {
    Flat,
    Plus,
    RevL,
    Vert,
    Square,
}

impl Rock {
    fn points(&self, base: &Point) -> Vec<Point> {
        match self {
            // ####
            Rock::Flat => vec![
                Point::new(base.x, base.y),
                Point::new(base.x + 1, base.y),
                Point::new(base.x + 2, base.y),
                Point::new(base.x + 3, base.y),
            ],
            //  #
            // ###
            //  #
            Rock::Plus => vec![
                Point::new(base.x + 1, base.y),
                Point::new(base.x, base.y + 1),
                Point::new(base.x + 1, base.y + 1),
                Point::new(base.x + 2, base.y + 1),
                Point::new(base.x + 1, base.y + 2),
            ],
            //   #
            //   #
            // ###
            Rock::RevL => vec![
                Point::new(base.x, base.y),
                Point::new(base.x + 1, base.y),
                Point::new(base.x + 2, base.y),
                Point::new(base.x + 2, base.y + 1),
                Point::new(base.x + 2, base.y + 2),
            ],
            // #
            // #
            // #
            // #
            Rock::Vert => vec![
                Point::new(base.x, base.y),
                Point::new(base.x, base.y + 1),
                Point::new(base.x, base.y + 2),
                Point::new(base.x, base.y + 3),
            ],
            // ##
            // ##
            Rock::Square => vec![
                Point::new(base.x, base.y),
                Point::new(base.x + 1, base.y),
                Point::new(base.x, base.y + 1),
                Point::new(base.x + 1, base.y + 1),
            ],
        }
    }
}

struct FallingRocks {
    rocks: Vec<Rock>,
    current: usize,
}

impl FallingRocks {
    fn new() -> Self {
        FallingRocks {
            rocks: vec![Rock::Flat, Rock::Plus, Rock::RevL, Rock::Vert, Rock::Square],
            current: 0,
        }
    }
}

impl Iterator for FallingRocks {
    type Item = Rock;

    fn next(&mut self) -> Option<Self::Item> {
        let rock = self.rocks[self.current];
        self.current += 1;
        // if it gets to the end, repeat
        if self.current == self.rocks.len() {
            self.current = 0;
        }
        Some(rock)
    }
}

// for identifying the periodic sequence
simple_struct!(PeriodicInfo; rock_delta: usize, height_delta: usize);
simple_struct!(SeqId; rock: usize, jet: usize);
simple_struct!(ChamberState; rock_number: usize, height: usize);

struct RockChamber {
    width: usize,
    rocks: FallingRocks,
    jets: Jets,
    num_rocks_simulated: usize,
    height: usize,
    rock_points: HashSet<Point>,
}

impl RockChamber {
    fn new(rocks: FallingRocks, jets: Jets) -> Self {
        RockChamber {
            width: 7,
            rocks,
            jets,
            num_rocks_simulated: 0,
            height: 0,
            rock_points: HashSet::new(),
        }
    }

    fn simulate_rocks(&mut self, num_rocks: usize) -> () {
        let previously_simulated = self.num_rocks_simulated;
        while self.num_rocks_simulated < num_rocks + previously_simulated {
            let rock = self.rocks.next().unwrap();
            // rock starts 2 units from the left wall, 3 units from highest point
            let mut rock_pt = Point::new(2, self.height + 3);
            loop {
                // rock is maybe pushed by jet, then falls one unit
                let jet_dir = self.jets.next().unwrap();
                if let Some(new_pt) = self.try_move_rock(rock, &rock_pt, jet_dir) {
                    rock_pt = new_pt;
                }
                if let Some(new_pt) = self.try_move_rock(rock, &rock_pt, Move::Down) {
                    rock_pt = new_pt;
                } else {
                    break;
                }
            }
            self.add_rock(rock, rock_pt);

            self.num_rocks_simulated += 1;
        }
    }

    // if movement is ok, return new coords, else None
    fn try_move_rock(&mut self, rock: Rock, from_pt: &Point, dir: Move) -> Option<Point> {
        if let Some(new_position) = from_pt.try_move(dir) {
            if self.position_is_ok(rock, &new_position) {
                return Some(new_position);
            }
        }
        None
    }

    fn position_is_ok(&self, rock: Rock, point: &Point) -> bool {
        for pt in rock.points(point) {
            if pt.x >= self.width {
                return false;
            }
            if self.rock_points.get(&pt).is_some() {
                return false;
            }
        }
        true
    }

    // add points from a rock
    fn add_rock(&mut self, rock: Rock, at_point: Point) -> () {
        for pt in rock.points(&at_point) {
            if pt.y >= self.height {
                // plus one, because at row 0, the height is one
                self.height = pt.y + 1;
            }
            self.rock_points.insert(pt);
        }
    }

    // for visualizing (and debugging) the first few rocks
    fn _print(&self) -> () {
        let mut rows: Vec<Vec<char>> = Vec::new();
        for _ in 0..20 {
            rows.push(vec!['.', '.', '.', '.', '.', '.', '.']);
        }
        for p in self.rock_points.iter() {
            rows[p.y][p.x] = '#';
        }
        rows.reverse();
        for r in rows {
            println!("|{}|", r.into_iter().collect::<String>());
        }
        println!("+-------+");
    }

    fn state(&self) -> ChamberState {
        ChamberState::new(self.num_rocks_simulated, self.height)
    }

    // for the big number in part2
    // find the period by matching the height delta and top points of each column
    fn simulate_rocks_big(&mut self, num_rocks: usize) -> () {
        let period_info = self.find_period();
        println!("{:?}", period_info);

        let num_rocks_left = num_rocks - self.num_rocks_simulated;
        let number_of_periods = num_rocks_left / period_info.rock_delta;
        let rocks_to_add_at_end = num_rocks_left % period_info.rock_delta;
        println!(
            "{} rocks left = {} periods + {} added at the end",
            num_rocks_left, number_of_periods, rocks_to_add_at_end
        );

        // use the period to skip most of the remaining rocks
        self.height += number_of_periods * period_info.height_delta;
        self.num_rocks_simulated += number_of_periods * period_info.rock_delta;
        self.copy_all_points(number_of_periods * period_info.height_delta);

        // the remaining rocks
        self.simulate_rocks(rocks_to_add_at_end);
    }

    fn find_period(&mut self) -> PeriodicInfo {
        // figure out the repetition of rock number and instruction number
        let mut sequence_combos: Vec<(SeqId, ChamberState)> = Vec::new();
        let mut sequences_seen: HashSet<SeqId> = HashSet::new();

        // example takes < 100 to figure this out,
        // full input takes ???
        let mut num_matched_in_a_row = 0;
        for _ in 0..10_000 {
            let seq = SeqId::new(self.rocks.current, self.jets.current);
            if let Some(_) = sequences_seen.get(&seq) {
                // once we've matched 10 in a row, that's good enough I guess ¯\_(ツ)_/¯
                if num_matched_in_a_row == 9 {
                    sequence_combos.push((seq.clone(), self.state()));
                    return self.calculate_period_info(sequence_combos);
                } else {
                    num_matched_in_a_row += 1;
                }
            } else {
                num_matched_in_a_row = 0;
            }
            sequence_combos.push((seq.clone(), self.state()));
            sequences_seen.insert(seq);
            self.simulate_rocks(1);
        }
        panic!("Could not determine repetition period");
    }

    // find the repeated sequence from the input Vec,
    // knowing that the last 10 are repeated somewhere in there
    fn calculate_period_info(&self, seq: Vec<(SeqId, ChamberState)>) -> PeriodicInfo {
        let last_10 = &seq[(seq.len() - 10)..seq.len()];
        last_10.iter().for_each(|e| println!("{:?}", e));

        // maybe there is a faster/cleaner way for this, I dunno ¯\_(ツ)_/¯
        for i in 0..(seq.len() - 10) {
            if seq[i].0 == last_10[0].0
                && seq[i + 1].0 == last_10[1].0
                && seq[i + 2].0 == last_10[2].0
                && seq[i + 3].0 == last_10[3].0
                && seq[i + 4].0 == last_10[4].0
                && seq[i + 5].0 == last_10[5].0
                && seq[i + 6].0 == last_10[6].0
                && seq[i + 7].0 == last_10[7].0
                && seq[i + 8].0 == last_10[8].0
                && seq[i + 9].0 == last_10[9].0
            {
                println!("Period starts at index {}: {:?}", i, seq[i]);
                return PeriodicInfo::new(
                    last_10[0].1.rock_number - seq[i].1.rock_number,
                    last_10[0].1.height - seq[i].1.height,
                );
            }
        }

        panic!("Could not find repeated sequence");
    }

    // copy all existing points from prev top to new top
    fn copy_all_points(&mut self, height_delta: usize) -> () {
        let points_to_add: Vec<Point> = self
            .rock_points
            .iter()
            .map(|p| Point::new(p.x, p.y + height_delta))
            .collect();

        for p in points_to_add.into_iter() {
            self.rock_points.insert(p);
        }
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let jets = parse_into_jets(&file_contents);
    let rocks = FallingRocks::new();
    let mut chamber = RockChamber::new(rocks, jets);
    chamber.simulate_rocks(2022);

    chamber.height
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let jets = parse_into_jets(&file_contents);
    let rocks = FallingRocks::new();
    let mut chamber = RockChamber::new(rocks, jets);
    chamber.simulate_rocks_big(1_000_000_000_000);

    chamber.height
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day17, part1, example, 3068);
    test_fn!(day17, part1, input, 3161);

    test_fn!(day17, part2, example, 1514285714288usize);
    test_fn!(day17, part2, input, 1575931232076usize);
}
