use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::space0;
use nom::character::complete::space1;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

use run_aoc::runner_fn;
use utils::{nom_usize, simple_struct};

// holds a single move instruction
simple_struct!(Move; quantity: usize, from: usize, to: usize);

struct Stacks<'a> {
    stacks: Vec<Vec<&'a str>>,
    size: usize,
}

impl<'a> Stacks<'a> {
    fn new(size: usize) -> Stacks<'a> {
        let mut stacks: Vec<Vec<&str>> = vec![];
        for _i in 0..size {
            stacks.push(vec![]);
        }
        Stacks { stacks, size }
    }

    // put crates in stacks, working backwards through the input
    fn initialize_crates(&mut self, crate_info: Vec<Vec<&'a str>>) -> () {
        crate_info.iter().rev().for_each(|crate_line| {
            //println!("{:?}", crate_line);
            crate_line.iter().enumerate().for_each(|(i, c)| {
                if c != &" " {
                    self.stacks[i].push(c);
                }
            });
        });
    }

    fn print(&self) {
        let max_stack_height = self
            .stacks
            .iter()
            .map(|s| s.len())
            .max()
            .expect("failed getting max height");
        //println!("max height: {}", max_stack_height);

        println!("");
        for i in (0..max_stack_height).rev() {
            for j in 0..self.size {
                if self.stacks[j].len() > i {
                    print!("[{}] ", self.stacks[j][i]);
                } else {
                    print!("    ");
                }
            }
            println!("");
        }
        for i in 0..self.size {
            print!(" {}  ", i + 1);
        }
        println!("");
        println!("");
    }

    fn get_tops(&self) -> String {
        (0..self.size)
            .map(|i| self.stacks[i][self.stacks[i].len() - 1])
            .collect::<Vec<&str>>()
            .join("")
    }

    fn do_moves(&mut self, vm: Vec<Move>) -> () {
        vm.iter().for_each(|mv| self.move_crates(mv));
    }

    fn move_crates(&mut self, m: &Move) -> () {
        println!("move {} crate(s) from {} to {}", m.quantity, m.from, m.to);
        // instructions are 1-based, vecs are 0-based
        let from_index = m.from - 1;
        let to_index = m.to - 1;
        for _i in 0..m.quantity {
            if let Some(c) = self.stacks[from_index].pop() {
                self.stacks[to_index].push(c);
            } else {
                println!(
                    "Stack {} is empty! Can't move another crate",
                    from_index + 1
                );
                panic!("Can't execute the move instruction");
            }
        }
        self.print();
    }

    fn do_moves_2(&mut self, vm: Vec<Move>) -> () {
        vm.iter().for_each(|mv| self.move_crates_2(mv));
    }

    // this way preserves the order of moved crates
    fn move_crates_2(&mut self, m: &Move) -> () {
        println!("move {} crate(s) from {} to {}", m.quantity, m.from, m.to);
        // instructions are 1-based, vecs are 0-based
        let from_index = m.from - 1;
        let to_index = m.to - 1;
        let mut temp_stack: Vec<&str> = vec![];

        for _i in 0..m.quantity {
            if let Some(c) = self.stacks[from_index].pop() {
                temp_stack.push(c);
            } else {
                println!(
                    "Stack {} is empty! Can't move another crate",
                    from_index + 1
                );
                panic!("Can't execute the move instruction");
            }
        }
        temp_stack.iter().rev().for_each(|c| {
            self.stacks[to_index].push(c);
        });
        self.print();
    }
}

#[derive(Debug)]
enum ParsedLine<'a> {
    Crates(Vec<&'a str>),
    StackNums(Vec<usize>),
    MoveInstr(Move),
}

// line could be one of:
//  - crates
//  - stack numbers
//  - move instruction
// NOTE: if there are >=10 stacks, the spacing gets weird, so assume <= 9 for now
fn parse_line(line: &str) -> IResult<&str, ParsedLine> {
    alt((crates, stack_nums, move_instr))(line)
}

fn crates(input: &str) -> IResult<&str, ParsedLine> {
    map(separated_list1(tag(" "), crate_or_empty), |result| {
        ParsedLine::Crates(result)
    })(input)
}

fn crate_or_empty(input: &str) -> IResult<&str, &str> {
    alt((
        delimited(tag(" "), tag(" "), tag(" ")),
        delimited(tag("["), alpha1, tag("]")),
    ))(input)
}

fn stack_nums(input: &str) -> IResult<&str, ParsedLine> {
    map(
        delimited(space0, separated_list1(space1, nom_usize), space0),
        |vec_of_stack_nums| ParsedLine::StackNums(vec_of_stack_nums),
    )(input)
}

fn move_instr(input: &str) -> IResult<&str, ParsedLine> {
    map(
        tuple((
            tag("move "),
            nom_usize,
            tag(" from "),
            nom_usize,
            tag(" to "),
            nom_usize,
        )),
        |(_m, d1, _f, d2, _t, d3)| ParsedLine::MoveInstr(Move::new(d1, d2, d3)),
    )(input)
}

#[runner_fn]
pub fn part1(file_contents: String) -> String {
    let mut crate_info: Vec<Vec<&str>> = vec![];
    let mut moves: Vec<Move> = vec![];
    // based on how many stacks are found when parsing
    let mut num_stacks = 0;

    file_contents.lines().for_each(|line| {
        //println!("line: {}", line);
        if line != "" {
            let (leftover, result) = parse_line(line).expect("failed to parse line");
            assert_eq!(leftover, "");
            //println!("result: {:?}", result);

            match result {
                ParsedLine::Crates(c) => {
                    if c.len() > num_stacks {
                        num_stacks = c.len();
                    }
                    crate_info.push(c);
                }
                ParsedLine::StackNums(nums) => {
                    // this should match the number of crates that were found
                    // (otherwise something is wrong with the parsing logic)
                    assert_eq!(nums.len(), num_stacks);
                }
                ParsedLine::MoveInstr(m) => {
                    moves.push(m);
                }
            }
        }
    });

    // construct the stacks, backwards, from the parsed info
    let mut stacks = Stacks::new(num_stacks);
    stacks.initialize_crates(crate_info);
    stacks.print();

    stacks.do_moves(moves);
    let tops = stacks.get_tops();
    println!("tops: {}", tops);
    tops
}

#[runner_fn]
pub fn part2(file_contents: String) -> String {
    let mut crate_info: Vec<Vec<&str>> = vec![];
    let mut moves: Vec<Move> = vec![];
    // based on how many stacks are found when parsing
    let mut num_stacks = 0;

    file_contents.lines().for_each(|line| {
        //println!("line: {}", line);
        if line != "" {
            let (leftover, result) = parse_line(line).expect("failed to parse line");
            assert_eq!(leftover, "");
            //println!("result: {:?}", result);

            match result {
                ParsedLine::Crates(c) => {
                    if c.len() > num_stacks {
                        num_stacks = c.len();
                    }
                    crate_info.push(c);
                }
                ParsedLine::StackNums(nums) => {
                    // this should match the number of crates that were found
                    // (otherwise something is wrong with the parsing logic)
                    assert_eq!(nums.len(), num_stacks);
                }
                ParsedLine::MoveInstr(m) => {
                    moves.push(m);
                }
            }
        }
    });

    // construct the stacks, backwards, from the parsed info
    let mut stacks = Stacks::new(num_stacks);
    stacks.initialize_crates(crate_info);
    stacks.print();

    stacks.do_moves_2(moves);
    let tops = stacks.get_tops();
    println!("tops: {}", tops);
    tops
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day5, part1, example, "CMZ");
    test_fn!(day5, part1, input, "TLNGFGMFN");

    test_fn!(day5, part2, example, "MCD");
    test_fn!(day5, part2, input, "FGLQJCMBD");
}
