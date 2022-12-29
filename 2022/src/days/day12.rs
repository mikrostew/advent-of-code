use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::multi::many1;
use nom::sequence::terminated;
use nom::IResult;

use super::simple_struct;
use run_aoc::runner_fn;

// TODO: generalize and extract this (will likely need it again)
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm
// Note: this only figures out the cost, it doesn't return the path
// (that's not needed for this problem)
struct Dijkstra<FF, FN> {
    unvisited: BinaryHeap<Node>,
    visited: HashSet<Point>,
    found: FF,
    neighbors: FN,
}

impl<FF, FN> Dijkstra<FF, FN>
where
    FF: FnMut(&Point) -> bool,
    FN: FnMut(&Point) -> Vec<Point>,
{
    fn new(start: Point, found: FF, neighbors: FN) -> Self {
        let mut unvisited = BinaryHeap::new();
        unvisited.push(Node { p: start, cost: 0 });

        Self {
            unvisited,
            visited: HashSet::new(),
            found,
            neighbors,
        }
    }

    fn solve(&mut self) -> Option<usize> {
        while let Some(Node { p, cost }) = self.unvisited.pop() {
            // don't re-visit nodes
            if self.visited.contains(&p) {
                continue;
            }
            self.visited.insert(p.clone());

            // if we found the target point, this will be the smallest cost
            // (since the unvisited head is sorted min-first)
            if (self.found)(&p) {
                return Some(cost);
            }

            // add all neighbors of this node to the unvisited heap
            for neighbor in (self.neighbors)(&p) {
                if !self.visited.contains(&neighbor) {
                    self.unvisited.push(Node {
                        p: neighbor,
                        cost: cost + 1,
                    });
                }
            }
        }
        // not able to solve
        None
    }
}

// TODO: this should be something more generic
struct Node {
    p: Point,
    cost: usize,
}

// have to implement this stuff to sort by cost in the BinaryHeap
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

simple_struct!(Point; x: usize, y: usize);

#[derive(Debug)]
struct Map {
    heights: Vec<Vec<u32>>,
}

impl Map {
    fn get_height(&self, p: &Point) -> u32 {
        self.heights[p.x][p.y]
    }

    // neighbors of the input point
    fn get_neighbors(&self, p: &Point) -> Vec<Point> {
        let mut neighbor_pts = vec![];
        if p.x > 0 {
            neighbor_pts.push(Point::new(p.x - 1, p.y));
        }
        if p.x < self.heights.len() - 1 {
            neighbor_pts.push(Point::new(p.x + 1, p.y));
        }
        if p.y > 0 {
            neighbor_pts.push(Point::new(p.x, p.y - 1));
        }
        if p.y < self.heights[0].len() - 1 {
            neighbor_pts.push(Point::new(p.x, p.y + 1));
        }
        neighbor_pts
    }
}

fn parse_map(input: &str) -> (Map, Point, Point) {
    let (leftover, chars) = parse_heights(input).expect("Could not parse heights!");
    assert_eq!(leftover, "");

    let mut start_point = Point::new(0, 0);
    let mut end_point = Point::new(0, 0);
    let heights = chars
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, c)| match c {
                    'S' => {
                        start_point = Point::new(x, y);
                        0
                    }
                    'E' => {
                        end_point = Point::new(x, y);
                        ('z' as u32) - ('a' as u32)
                    }
                    // already validated rest of chars are in range a-z with one_of()
                    _ => (*c as u32) - ('a' as u32),
                })
                .collect()
        })
        .collect();
    (Map { heights }, start_point, end_point)
}

fn parse_heights(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(parse_line)(input)
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of("abcdefghijklmnopqrstuvwxyzSE")), newline)(input)
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let (map, start, end) = parse_map(&file_contents);

    let mut solver = Dijkstra::new(
        start,
        |p| *p == end,
        |p| {
            let current_height = map.get_height(p);

            map.get_neighbors(p)
                .into_iter()
                .filter_map(|n| {
                    if map.get_height(&n) <= current_height + 1 {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        },
    );

    let cost = solver.solve().expect("Could not solve!");
    cost
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let (map, _start, end) = parse_map(&file_contents);

    // this time start at the end, and find the fewest steps to get to a point of height 0
    let mut solver = Dijkstra::new(
        end,
        |p| map.get_height(p) == 0,
        |p| {
            let current_height = map.get_height(p);

            map.get_neighbors(p)
                .into_iter()
                .filter_map(|n| {
                    if map.get_height(&n) >= current_height - 1 {
                        Some(n)
                    } else {
                        None
                    }
                })
                .collect::<Vec<Point>>()
        },
    );

    let cost = solver.solve().expect("Could not solve!");
    cost
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day12, part1, example, 31);
    test_fn!(day12, part1, input, 361);

    test_fn!(day12, part2, example, 29);
    test_fn!(day12, part2, input, 354);
}
