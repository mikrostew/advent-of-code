use nom::branch::alt;
// use nom::bytes::complete::tag;
// use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
// use nom::combinator::opt;
use nom::multi::many1;
use nom::multi::separated_list1;
// use nom::sequence::preceded;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::parse_usize;
use super::simple_struct;

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
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
                Move::Right => {
                    self.direction = self.direction.turn(mv);
                }
                Move::Left => {
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

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, Path) {
    let (leftover, (tiles, path)) = parse_board_and_path(input).expect("couldn't parse input");
    assert_eq!(leftover, "");
    (tiles, path)
}

pub fn part1(file_contents: String) -> String {
    let (tiles, path) = parse_input(&file_contents);
    let mut board = Board2D::from(tiles);
    board.follow_path(&path);

    let position = board.position;
    let direction = board.direction;
    let password = 1000 * (position.row + 1) + 4 * (position.col + 1) + direction.facing();

    format!("{}", password)
}

pub fn part2(file_contents: String) -> String {
    println!("{}", file_contents);
    "TODO".to_string()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day22-example.txt");
        assert_eq!(part1(input), "6032".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day22-input.txt");
        assert_eq!(part1(input), "43466".to_string());
    }

    // #[test]
    // fn part2_example() {
    //     let input = read_input_file("inputs/day21-example.txt");
    //     assert_eq!(part2(input), "301".to_string());
    // }

    // #[test]
    // fn part2_input() {
    //     let input = read_input_file("inputs/day21-input.txt");
    //     assert_eq!(part2(input), "3848301405790".to_string());
    // }
}
