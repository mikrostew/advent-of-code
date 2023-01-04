use std::collections::{HashMap, HashSet, VecDeque};

use nom::branch::alt;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::many1;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::gcd;
use super::parse_usize;
use super::simple_struct;
use run_aoc::runner_fn;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Tile {
    // nothing there
    Empty,
    // can move on this
    Open,
    // cannot move to this
    Wall,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
enum Move {
    // move this many tiles in the current direction
    Num(usize),
    // turn 90 deg clockwise
    Right,
    // turn 90 deg counter-clockwise
    Left,
}

simple_struct!(Path; moves: Vec<Move>);

fn parse_tile(input: &str) -> IResult<&str, Tile> {
    map(one_of(" .#"), |t| match t {
        ' ' => Tile::Empty,
        '.' => Tile::Open,
        '#' => Tile::Wall,
        _ => panic!("shouldn't happen"),
    })(input)
}

fn parse_row(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(parse_tile)(input)
}

fn parse_board(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(newline, parse_row)(input)
}

fn parse_move_num(input: &str) -> IResult<&str, Move> {
    map(parse_usize, |n| Move::Num(n))(input)
}

fn parse_move_dir(input: &str) -> IResult<&str, Move> {
    map(one_of("LR"), |m| match m {
        'L' => Move::Left,
        'R' => Move::Right,
        _ => panic!("shouldn't happen"),
    })(input)
}

fn parse_move(input: &str) -> IResult<&str, Move> {
    alt((parse_move_num, parse_move_dir))(input)
}

fn parse_path(input: &str) -> IResult<&str, Path> {
    map(many1(parse_move), |vm| Path::new(vm))(input)
}

fn parse_board_and_path(input: &str) -> IResult<&str, (Vec<Vec<Tile>>, Path)> {
    terminated(
        separated_pair(parse_board, tuple((newline, newline)), parse_path),
        newline,
    )(input)
}

simple_struct!(Point; row: usize, col: usize);

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Dir {
    Right,
    Down,
    Left,
    Up,
}

impl Dir {
    fn facing(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn turn(&self, m: &Move) -> Self {
        match m {
            Move::Right => match self {
                Self::Right => Self::Down,
                Self::Down => Self::Left,
                Self::Left => Self::Up,
                Self::Up => Self::Right,
            },
            Move::Left => match self {
                Self::Right => Self::Up,
                Self::Down => Self::Right,
                Self::Left => Self::Down,
                Self::Up => Self::Left,
            },
            _ => panic!("don't do that"),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
        }
    }

    fn iterator() -> impl Iterator<Item = Dir> {
        [Dir::Right, Dir::Down, Dir::Left, Dir::Up].iter().copied()
    }
}

struct Board2D {
    position: Point,
    direction: Dir,
    tiles: Vec<Vec<Tile>>,
}

impl Board2D {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        let position = Board2D::find_initial_position(&tiles);
        Board2D {
            position,
            direction: Dir::Right,
            tiles,
        }
    }

    // find the leftmost open tile of the top row of tiles
    fn find_initial_position(tiles: &Vec<Vec<Tile>>) -> Point {
        let row = 0;
        let first_row = &tiles[row];
        for col in 0..first_row.len() {
            if first_row[col] == Tile::Open {
                return Point::new(row, col);
            }
        }
        panic!("no open file found");
    }

    fn follow_path(&mut self, path: &Path) -> () {
        for mv in path.moves.iter() {
            match mv {
                Move::Num(n) => self.do_move(*n),
                Move::Right | Move::Left => {
                    self.direction = self.direction.turn(mv);
                }
            }
        }
    }

    fn do_move(&mut self, num_tiles: usize) -> () {
        for _ in 0..num_tiles {
            self.single_step();
        }
    }

    fn single_step(&mut self) -> () {
        let next_pos = match self.direction {
            // stay in same row, col+1
            Dir::Right => {
                if self.position.col + 1 == self.tiles[self.position.row].len() {
                    self.wraparound_row(0..self.position.col)
                } else {
                    Point::new(self.position.row, self.position.col + 1)
                }
            }
            // stay in same col, row+1
            Dir::Down => {
                if self.position.row + 1 >= self.tiles.len()
                    || self.position.col >= self.tiles[self.position.row + 1].len()
                    || self.tiles[self.position.row + 1][self.position.col] == Tile::Empty
                {
                    self.wraparound_col(0..self.position.row)
                } else {
                    Point::new(self.position.row + 1, self.position.col)
                }
            }
            // stay in same row, col-1
            Dir::Left => {
                if self.position.col == 0
                    || self.tiles[self.position.row][self.position.col - 1] == Tile::Empty
                {
                    // wraparound
                    Point::new(self.position.row, self.tiles[self.position.row].len() - 1)
                } else {
                    Point::new(self.position.row, self.position.col - 1)
                }
            }
            // stay in same col, row-1
            Dir::Up => {
                if self.position.row == 0
                    || self.tiles[self.position.row - 1][self.position.col] == Tile::Empty
                {
                    self.wraparound_col((self.position.row..self.tiles.len()).rev())
                } else {
                    Point::new(self.position.row - 1, self.position.col)
                }
            }
        };
        if self.tiles[next_pos.row][next_pos.col] == Tile::Open {
            self.position = next_pos;
        }
    }

    fn wraparound_row(&self, cols: impl Iterator<Item = usize>) -> Point {
        for x in cols {
            if self.tiles[self.position.row][x] != Tile::Empty {
                return Point::new(self.position.row, x);
            }
        }
        panic!("not able to wrap around row");
    }

    fn wraparound_col(&self, rows: impl Iterator<Item = usize>) -> Point {
        for y in rows {
            // have to make sure the row is long enough, to avoid OOB
            if self.position.col < self.tiles[y].len() {
                if self.tiles[y][self.position.col] != Tile::Empty {
                    return Point::new(y, self.position.col);
                }
            }
        }
        panic!("not able to wrap around col");
    }
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum FaceID {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

impl FaceID {
    fn from_num(n: usize) -> Self {
        match n {
            1 => FaceID::One,
            2 => FaceID::Two,
            3 => FaceID::Three,
            4 => FaceID::Four,
            5 => FaceID::Five,
            6 => FaceID::Six,
            _ => panic!("cube face out of range"),
        }
    }

    fn iterator() -> impl Iterator<Item = FaceID> {
        [
            FaceID::One,
            FaceID::Two,
            FaceID::Three,
            FaceID::Four,
            FaceID::Five,
            FaceID::Six,
        ]
        .iter()
        .copied()
    }
}

struct CubeFace {
    // location of the top left point in the input
    top_left: Point,
    tiles: Vec<Vec<Tile>>,
    // map outgoing dir to neighbor face ID and incoming dir
    neighbors: HashMap<Dir, (FaceID, Dir)>,
    position: Point,
    direction: Dir,
}

impl CubeFace {
    fn new(top_left: Point, tiles: Vec<Vec<Tile>>) -> Self {
        CubeFace {
            top_left,
            tiles,
            neighbors: HashMap::new(),
            // these only matter for FaceID::One at the start
            position: Point::new(0, 0),
            direction: Dir::Right,
        }
    }
}

struct Board3D {
    cube_size: usize,
    faces: HashMap<FaceID, CubeFace>,
    face_id: FaceID,
}

impl Board3D {
    fn from(tiles: Vec<Vec<Tile>>) -> Self {
        let cube_size = Board3D::find_cube_size(&tiles);
        let faces = Board3D::figure_out_faces(tiles, cube_size);

        Board3D {
            cube_size,
            faces,
            // always start with this face
            face_id: FaceID::One,
        }
    }

    // use GCD to figure out the size of each cube face
    fn find_cube_size(tiles: &Vec<Vec<Tile>>) -> usize {
        let row_lengths: HashSet<usize> = tiles.iter().map(|r| r.len()).collect();
        let length_vec: Vec<usize> = row_lengths.into_iter().collect();
        gcd(length_vec[0], length_vec[1])
    }

    fn figure_out_faces(tiles: Vec<Vec<Tile>>, cube_size: usize) -> HashMap<FaceID, CubeFace> {
        let mut faces: HashMap<FaceID, CubeFace> = HashMap::new();
        let mut top_left_map: HashMap<Point, FaceID> = HashMap::new();
        let mut cube_number = 1;

        // figure out where the faces are from the input data
        let num_rows = tiles.len();
        for r in (0..num_rows).step_by(cube_size) {
            let num_cols = tiles[r].len();
            for c in (0..num_cols).step_by(cube_size) {
                match tiles[r][c] {
                    Tile::Empty => {
                        // keep going
                    }
                    Tile::Open | Tile::Wall => {
                        let cube_tiles: Vec<Vec<Tile>> = (r..(r + cube_size))
                            .map(|row| {
                                let mut x = vec![Tile::Empty; cube_size];
                                x[..cube_size].copy_from_slice(&tiles[row][c..(c + cube_size)]);
                                x
                            })
                            .collect();
                        faces.insert(
                            FaceID::from_num(cube_number),
                            CubeFace::new(Point::new(r, c), cube_tiles),
                        );
                        // and add the reverse mapping
                        top_left_map.insert(Point::new(r, c), FaceID::from_num(cube_number));
                        cube_number += 1;
                    }
                }
            }
        }

        // figure out how the faces are oriented in relation to each other in the input
        let mut face_queue: VecDeque<(FaceID, Dir)> = VecDeque::new();
        for f in FaceID::iterator() {
            for d in Dir::iterator() {
                let current_face_point = &faces.get(&f).unwrap().top_left;
                let (neighbor_row, neighbor_col, neighbor_dir) = match d {
                    Dir::Left => (
                        current_face_point.row,
                        if current_face_point.col == 0 {
                            usize::MAX
                        } else {
                            current_face_point.col - cube_size
                        },
                        Dir::Right,
                    ),
                    Dir::Up => (
                        if current_face_point.row == 0 {
                            usize::MAX
                        } else {
                            current_face_point.row - cube_size
                        },
                        current_face_point.col,
                        Dir::Down,
                    ),
                    Dir::Right => (
                        current_face_point.row,
                        current_face_point.col + cube_size,
                        Dir::Left,
                    ),
                    Dir::Down => (
                        current_face_point.row + cube_size,
                        current_face_point.col,
                        Dir::Up,
                    ),
                };
                if let Some(neighbor_face_id) =
                    top_left_map.get(&Point::new(neighbor_row, neighbor_col))
                {
                    faces
                        .get_mut(&f)
                        .unwrap()
                        .neighbors
                        .insert(d, (*neighbor_face_id, neighbor_dir));
                } else {
                    face_queue.push_back((f, d));
                }
            }
        }

        // figure out the rest of the orientations based on that
        while let Some((face_id, dir)) = face_queue.pop_front() {
            if let Some((neighbor_id, incoming_dir)) =
                Board3D::find_face_in_dir(&faces, face_id, dir)
            {
                // found the neighbor, so do the insert
                let current_face = faces.get_mut(&face_id).unwrap();
                current_face
                    .neighbors
                    .insert(dir, (neighbor_id, incoming_dir));
            } else {
                // not yet, try again
                face_queue.push_back((face_id, dir));
            }
        }

        faces
    }

    fn find_face_in_dir(
        faces: &HashMap<FaceID, CubeFace>,
        face_id: FaceID,
        dir: Dir,
    ) -> Option<(FaceID, Dir)> {
        let current_face = faces.get(&face_id).unwrap();
        let dirs_to_check: Vec<(Dir, Move)> = match dir {
            // for each direction
            // - do I have a neighbor, and
            // - does that neighbor have a neighbor in the right direction?
            Dir::Left => vec![(Dir::Down, Move::Left), (Dir::Up, Move::Right)],
            Dir::Up => vec![(Dir::Left, Move::Left), (Dir::Right, Move::Right)],
            Dir::Right => vec![(Dir::Up, Move::Left), (Dir::Down, Move::Right)],
            Dir::Down => vec![(Dir::Right, Move::Left), (Dir::Left, Move::Right)],
        };

        for (neighbor_dir, turn_move) in dirs_to_check.iter() {
            if let Some((neighbor_id, incoming_dir)) = current_face.neighbors.get(neighbor_dir) {
                let neighbor = faces.get(neighbor_id).unwrap();
                let out_dir = incoming_dir.turn(turn_move);
                if let Some((nbor_out_id, nbor_incoming_dir)) = neighbor.neighbors.get(&out_dir) {
                    // found it
                    let some_incoming_dir = nbor_incoming_dir.turn(turn_move);
                    return Some((*nbor_out_id, some_incoming_dir));
                }
            }
        }
        None
    }

    fn follow_path(&mut self, path: &Path) -> () {
        for mv in path.moves.iter() {
            match mv {
                Move::Num(n) => self.do_move(*n),
                Move::Right | Move::Left => {
                    let current_face = self.faces.get_mut(&self.face_id).unwrap();
                    current_face.direction = current_face.direction.turn(mv);
                }
            }
        }
    }

    fn do_move(&mut self, num_tiles: usize) -> () {
        for _ in 0..num_tiles {
            self.single_step();
        }
    }

    fn single_step(&mut self) -> () {
        let cf = self.faces.get(&self.face_id).unwrap();

        // only do this if I need to change faces?
        let (new_position, maybe_new_face_and_direction) = match cf.direction {
            // stay in same row, col+1
            Dir::Right => {
                if cf.position.col + 1 == cf.tiles[cf.position.row].len() {
                    self.map_to_neighbor(&cf, Dir::Right, cf.position.row)
                } else {
                    (Point::new(cf.position.row, cf.position.col + 1), None)
                }
            }
            // stay in same col, row+1
            Dir::Down => {
                if cf.position.row + 1 == cf.tiles.len() {
                    self.map_to_neighbor(&cf, Dir::Down, cf.position.col)
                } else {
                    (Point::new(cf.position.row + 1, cf.position.col), None)
                }
            }
            // stay in same row, col-1
            Dir::Left => {
                if cf.position.col == 0 {
                    self.map_to_neighbor(&cf, Dir::Left, cf.position.row)
                } else {
                    (Point::new(cf.position.row, cf.position.col - 1), None)
                }
            }
            // stay in same col, row-1
            Dir::Up => {
                if cf.position.row == 0 {
                    self.map_to_neighbor(&cf, Dir::Up, cf.position.col)
                } else {
                    (Point::new(cf.position.row - 1, cf.position.col), None)
                }
            }
        };

        if let Some((new_face_id, new_dir)) = maybe_new_face_and_direction {
            let neighbor_face = self.faces.get(&new_face_id).unwrap();
            if neighbor_face.tiles[new_position.row][new_position.col] == Tile::Open {
                // move to a different face
                self.face_id = new_face_id;
                let mut_face = self.faces.get_mut(&new_face_id).unwrap();
                mut_face.position = new_position;
                mut_face.direction = new_dir;
            }
        } else {
            // stay on the same face
            if cf.tiles[new_position.row][new_position.col] == Tile::Open {
                let mut_face = self.faces.get_mut(&self.face_id).unwrap();
                mut_face.position = new_position;
            }
        }
    }

    // figure out what point and direction this would be on the neighboring face
    fn map_to_neighbor(
        &self,
        current_face: &CubeFace,
        outgoing_dir: Dir,
        incoming_pos: usize,
    ) -> (Point, Option<(FaceID, Dir)>) {
        let (neighbor_face_id, incoming_dir) = current_face.neighbors.get(&outgoing_dir).unwrap();
        let facing_dir = incoming_dir.opposite();
        let flipped_pos = self.cube_size - 1 - incoming_pos;
        let end_pos = self.cube_size - 1;
        // maybe a better/simpler way to do this ¯\_(ツ)_/¯
        // (also, some of these combos may not be possible)
        let new_point = match outgoing_dir {
            Dir::Left | Dir::Down => match incoming_dir {
                Dir::Left => Point::new(flipped_pos, 0),
                Dir::Up => Point::new(0, incoming_pos),
                Dir::Right => Point::new(incoming_pos, end_pos),
                Dir::Down => Point::new(end_pos, flipped_pos),
            },
            Dir::Up | Dir::Right => match incoming_dir {
                Dir::Left => Point::new(incoming_pos, 0),
                Dir::Up => Point::new(0, flipped_pos),
                Dir::Right => Point::new(flipped_pos, end_pos),
                Dir::Down => Point::new(end_pos, incoming_pos),
            },
        };
        (new_point, Some((*neighbor_face_id, facing_dir)))
    }

    fn position(&self) -> Point {
        let cf = self.faces.get(&self.face_id).unwrap();
        let top_left = cf.top_left.clone();
        let cf_pos = cf.position.clone();
        Point::new(top_left.row + cf_pos.row, top_left.col + cf_pos.col)
    }

    fn direction(&self) -> Dir {
        let cf = self.faces.get(&self.face_id).unwrap();
        cf.direction
    }
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Path) {
    let (leftover, (tiles, path)) = parse_board_and_path(input).expect("couldn't parse input");
    assert_eq!(leftover, "");
    (tiles, path)
}

fn password(position: Point, dir: Dir) -> usize {
    1000 * (position.row + 1) + 4 * (position.col + 1) + dir.facing()
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    let (tiles, path) = parse_input(&file_contents);
    let mut board = Board2D::from(tiles);
    board.follow_path(&path);

    password(board.position, board.direction)
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let (tiles, path) = parse_input(&file_contents);
    let mut board = Board3D::from(tiles);
    board.follow_path(&path);

    password(board.position(), board.direction())
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day22, part1, example, 6032);
    test_fn!(day22, part1, input, 43466);

    test_fn!(day22, part2, example, 5031);
    test_fn!(day22, part2, input, 162155);
}
