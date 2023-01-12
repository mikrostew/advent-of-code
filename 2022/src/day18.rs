use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

use nom::bytes::complete::tag;
use nom::character::complete::newline;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use run_aoc::runner_fn;
use utils::{nom_i32, simple_struct};

simple_struct!(Point; x: i32, y: i32, z: i32);

fn point_xyz(input: &str) -> IResult<&str, Point> {
    map(
        tuple((nom_i32, tag(","), nom_i32, tag(","), nom_i32)),
        |(x, _, y, _, z)| Point::new(x, y, z),
    )(input)
}

fn parse_points(input: &str) -> Vec<Point> {
    let (leftover, points) = terminated(separated_list1(newline, point_xyz), newline)(input)
        .expect("Could not parse input");
    assert_eq!(leftover, "");
    points
}

// state of each 1x1x1 unit in the grid
#[derive(Debug)]
enum GridUnit {
    Empty,
    Filled,
}

struct PointGrid {
    grid: HashMap<Point, GridUnit>,
    points: Vec<Point>,
}

impl PointGrid {
    fn from_points(points: Vec<Point>) -> Self {
        let (xmin, xmax, ymin, ymax, zmin, zmax) = PointGrid::find_limits_xyz(&points);
        println!(
            "Limits: X min={},max={} Y min={},max={} Z min={},max={}",
            xmin, xmax, ymin, ymax, zmin, zmax
        );
        let grid = PointGrid::build_point_grid(&points, xmin, xmax, ymin, ymax, zmin, zmax);
        println!("Built grid of {} points", grid.len());
        PointGrid { grid, points }
    }

    fn find_limits_xyz(points: &Vec<Point>) -> (i32, i32, i32, i32, i32, i32) {
        let (mut xmin, mut xmax, mut ymin, mut ymax, mut zmin, mut zmax) =
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN, i32::MAX, i32::MIN);
        for p in points.iter() {
            xmin = xmin.min(p.x);
            xmax = xmax.max(p.x);
            ymin = ymin.min(p.y);
            ymax = ymax.max(p.y);
            zmin = zmin.min(p.z);
            zmax = zmax.max(p.z);
        }
        (xmin, xmax, ymin, ymax, zmin, zmax)
    }

    fn build_point_grid(
        points: &Vec<Point>,
        xmin: i32,
        xmax: i32,
        ymin: i32,
        ymax: i32,
        zmin: i32,
        zmax: i32,
    ) -> HashMap<Point, GridUnit> {
        let mut point_grid: HashMap<Point, GridUnit> = HashMap::new();
        // start with all empty space
        // (extending one unit past the lava drop)
        for x in (xmin - 1)..=(xmax + 1) {
            for y in (ymin - 1)..=(ymax + 1) {
                for z in (zmin - 1)..=(zmax + 1) {
                    point_grid.insert(Point::new(x, y, z), GridUnit::Empty);
                }
            }
        }
        // then fill in the points
        for p in points.iter() {
            point_grid.insert(p.clone(), GridUnit::Filled);
        }
        point_grid
    }

    fn find_surface_area_all(&self) -> usize {
        let mut surface_area = 0;
        // just check all the input points for this
        for p in self.points.iter() {
            // check all the neighbors of this point
            let neighbors: Vec<Point> = vec![
                Point::new(p.x + 1, p.y, p.z),
                Point::new(p.x - 1, p.y, p.z),
                Point::new(p.x, p.y + 1, p.z),
                Point::new(p.x, p.y - 1, p.z),
                Point::new(p.x, p.y, p.z + 1),
                Point::new(p.x, p.y, p.z - 1),
            ];

            neighbors.into_iter().for_each(|n| match self.grid.get(&n) {
                Some(GridUnit::Empty) => {
                    // that's one surface in that direction
                    surface_area += 1;
                }
                Some(GridUnit::Filled) => {}
                // outside the grid, ignore it (shouldn't happen tho)
                None => {}
            });
        }

        surface_area
    }

    fn find_surface_area_ext(&self) -> usize {
        let mut surface_area = 0;
        let mut point_queue: VecDeque<Point> = VecDeque::new();
        let mut checked_points: HashSet<Point> = HashSet::new();
        // start at 0,0,0 and go from there
        point_queue.push_back(Point::new(0, 0, 0));
        checked_points.insert(Point::new(0, 0, 0));
        while let Some(p) = point_queue.pop_front() {
            // check all the neighbors of this point
            let neighbors: Vec<Point> = vec![
                Point::new(p.x + 1, p.y, p.z),
                Point::new(p.x - 1, p.y, p.z),
                Point::new(p.x, p.y + 1, p.z),
                Point::new(p.x, p.y - 1, p.z),
                Point::new(p.x, p.y, p.z + 1),
                Point::new(p.x, p.y, p.z - 1),
            ];

            neighbors.into_iter().for_each(|n| match self.grid.get(&n) {
                Some(GridUnit::Empty) => {
                    if !checked_points.get(&n).is_some() {
                        point_queue.push_back(n.clone());
                        checked_points.insert(n);
                    }
                }
                Some(GridUnit::Filled) => {
                    // that's one surface in that direction
                    surface_area += 1;
                }
                // outside the grid, ignore it
                None => {}
            });
        }
        println!("Checked {} points", checked_points.len());

        surface_area
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    let points = parse_points(&file_contents);
    println!("Parsed {} points", points.len());
    let point_grid = PointGrid::from_points(points);
    let area = point_grid.find_surface_area_all();

    area
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let points = parse_points(&file_contents);
    println!("Parsed {} points", points.len());
    let point_grid = PointGrid::from_points(points);
    let area = point_grid.find_surface_area_ext();

    area
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day18, part1, example, 64);
    test_fn!(day18, part1, input, 4512);

    test_fn!(day18, part2, example, 58);
    test_fn!(day18, part2, input, 2554);
}
