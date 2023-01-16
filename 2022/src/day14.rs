use std::collections::HashMap;

use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;

use run_aoc::runner_fn;
use utils::{nom_usize, simple_struct};

simple_struct!(Point; x: usize, y: usize);

#[derive(Debug)]
struct RockPath {
    path: Vec<Point>,
}

impl RockPath {
    fn parse(input: &str) -> Self {
        let (leftover, path) = rock_path(input).expect("Could not parse rock path");
        assert_eq!(leftover, "");
        RockPath { path }
    }
}

fn rock_path(input: &str) -> IResult<&str, Vec<Point>> {
    separated_list1(tag(" -> "), point)(input)
}

fn point(input: &str) -> IResult<&str, Point> {
    map(separated_pair(nom_usize, tag(","), nom_usize), |(x, y)| {
        Point::new(x, y)
    })(input)
}

// what is at each point?
// (not including Air, that's the default empty state)
enum Tile {
    Rock,
    Sand,
}

struct Cave {
    // map of points to what is at the point
    // (using this instead of a full 2D grid, because there are sparse points)
    points: HashMap<Point, Tile>,
    // lowest rock tile in the map
    lowest_y: usize,
}

impl Cave {
    fn new() -> Self {
        Cave {
            points: HashMap::new(),
            lowest_y: 0,
        }
    }

    fn add_rocks(&mut self, rp: &RockPath) -> () {
        // do this in pairs
        for i in 0..(rp.path.len() - 1) {
            let p1 = &rp.path[i];
            let p2 = &rp.path[i + 1];

            if p1.x == p2.x {
                let x = p1.x;
                if p1.y < p2.y {
                    for y in p1.y..=p2.y {
                        self.add_rock(x, y);
                    }
                } else {
                    for y in p2.y..=p1.y {
                        self.add_rock(x, y);
                    }
                }
            } else if p1.y == p2.y {
                let y = p1.y;
                if p1.x < p2.x {
                    for x in p1.x..=p2.x {
                        self.add_rock(x, y);
                    }
                } else {
                    for x in p2.x..=p1.x {
                        self.add_rock(x, y);
                    }
                }
            } else {
                panic!("Cannot make line from input points");
            }
        }
    }

    // add a single rock tile, tracking where the lowest tile is
    // (to later determine if sand is falling forever)
    fn add_rock(&mut self, x: usize, y: usize) -> () {
        self.points.insert(Point::new(x, y), Tile::Rock);
        if y > self.lowest_y {
            self.lowest_y = y;
        }
    }

    fn add_sand(&mut self, x: usize, y: usize) -> () {
        self.points.insert(Point::new(x, y), Tile::Sand);
    }

    // is this point taken by rock or sand?
    fn point_taken(&self, x: usize, y: usize) -> bool {
        self.points.get(&Point::new(x, y)).is_some()
    }

    // drop sand into the cave from the input point,
    // returning the point where the sand came to rest (None if it falls forever)
    fn drop_sand(&mut self, from_point: Point) -> Option<Point> {
        let mut current_point = from_point.clone();

        loop {
            // order to try
            // - down one
            // - diagonal left
            // - diagonal right
            if !self.point_taken(current_point.x, current_point.y + 1) {
                current_point.y += 1;
                // has the sand fallen past the lowest rock?
                if current_point.y > self.lowest_y {
                    return None;
                }
            } else if !self.point_taken(current_point.x - 1, current_point.y + 1) {
                current_point.x -= 1;
                current_point.y += 1;
            } else if !self.point_taken(current_point.x + 1, current_point.y + 1) {
                current_point.x += 1;
                current_point.y += 1;
            } else {
                // sand is at rest
                self.add_sand(current_point.x, current_point.y);
                return Some(current_point);
            }
        }
    }

    // drop sand into the cave from the input point,
    // returning the point where the sand came to rest
    // (this time there is an infinite floor at lowest point + 2)
    fn drop_sand_with_floor(&mut self, from_point: Point) -> Option<Point> {
        let floor_y = self.lowest_y + 2;
        let mut current_point = from_point.clone();

        loop {
            // order to try
            // - on floor (one grid above the floor tiles)
            // - down one
            // - diagonal left
            // - diagonal right
            if current_point.y == floor_y - 1 {
                // sand is at rest, on the floor
                self.add_sand(current_point.x, current_point.y);
                return Some(current_point);
            } else if !self.point_taken(current_point.x, current_point.y + 1) {
                current_point.y += 1;
            } else if !self.point_taken(current_point.x - 1, current_point.y + 1) {
                current_point.x -= 1;
                current_point.y += 1;
            } else if !self.point_taken(current_point.x + 1, current_point.y + 1) {
                current_point.x += 1;
                current_point.y += 1;
            } else {
                // sand is at rest
                self.add_sand(current_point.x, current_point.y);
                return Some(current_point);
            }
        }
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let rock_paths: Vec<RockPath> = file_contents.lines().map(RockPath::parse).collect();
    let mut cave = Cave::new();
    for rp in rock_paths.iter() {
        cave.add_rocks(rp);
    }

    let mut num_sand_grains = 0;
    while let Some(_p) = cave.drop_sand(Point::new(500, 0)) {
        num_sand_grains += 1;
        //println!("{}, {:?}", num_sand_grains, p);
    }

    num_sand_grains
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let rock_paths: Vec<RockPath> = file_contents.lines().map(RockPath::parse).collect();
    let mut cave = Cave::new();
    for rp in rock_paths.iter() {
        cave.add_rocks(rp);
    }

    let mut num_sand_grains = 0;
    while let Some(p) = cave.drop_sand_with_floor(Point::new(500, 0)) {
        num_sand_grains += 1;
        //println!("{}, {:?}", num_sand_grains, p);
        if p.x == 500 && p.y == 0 {
            break;
        }
    }

    num_sand_grains
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day14, part1, example, 24);
    test_fn!(day14, part1, input, 757);

    test_fn!(day14, part2, example, 93);
    test_fn!(day14, part2_SLOW, input, 24943);
}
