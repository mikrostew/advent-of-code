use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::sequence::terminated;
use nom::sequence::tuple;
use nom::IResult;

use super::parse_usize;
use crate::cli::Params;

// monkey language grammar

struct Assign(String, Expr);

enum Expr {
    Const(usize),
    BinaryOp(String, Op, String),
}

enum Op {
    Plus,
    Minus,
    Mult,
    Div,
    Equal,
}

// parse monkey language

fn parse_assign(input: &str) -> IResult<&str, Assign> {
    map(
        separated_pair(alpha1, tag(": "), parse_expr),
        |(var, expr)| Assign(String::from(var), expr),
    )(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((parse_const, parse_binary_op))(input)
}

fn parse_const(input: &str) -> IResult<&str, Expr> {
    map(parse_usize, |n| Expr::Const(n))(input)
}

fn parse_binary_op(input: &str) -> IResult<&str, Expr> {
    map(
        tuple((alpha1, tag(" "), one_of("+-*/"), tag(" "), alpha1)),
        |(v1, _, op, _, v2)| match op {
            '+' => Expr::BinaryOp(String::from(v1), Op::Plus, String::from(v2)),
            '-' => Expr::BinaryOp(String::from(v1), Op::Minus, String::from(v2)),
            '*' => Expr::BinaryOp(String::from(v1), Op::Mult, String::from(v2)),
            '/' => Expr::BinaryOp(String::from(v1), Op::Div, String::from(v2)),
            _ => panic!("shouldn't get here"),
        },
    )(input)
}

fn parse_lines(input: &str) -> Vec<Assign> {
    let (leftover, assigns) = terminated(separated_list1(newline, parse_assign), newline)(input)
        .expect("Could not parse lines");
    assert_eq!(leftover, "");
    assigns
}

struct Program {
    vars: HashMap<String, Expr>,
}

impl Program {
    fn new() -> Self {
        Program {
            vars: HashMap::new(),
        }
    }

    fn load_statements(&mut self, stmts: Vec<Assign>) -> () {
        for s in stmts.into_iter() {
            let Assign(var, expr) = s;
            self.vars.insert(var, expr);
        }
    }

    // for part2, make some adjustments
    fn load_statements_2(&mut self, stmts: Vec<Assign>) -> () {
        for s in stmts.into_iter() {
            let Assign(var, expr) = s;
            if var == "root" {
                if let Expr::BinaryOp(v1, _, v2) = expr {
                    self.vars.insert(var, Expr::BinaryOp(v1, Op::Equal, v2));
                }
            } else if var == "humn" {
                // don't insert this one, and see what happens...
            } else {
                self.vars.insert(var, expr);
            }
        }
    }

    fn evaluate(&self, var: &String) -> Result<usize, String> {
        let expr = self.vars.get(var).ok_or("var not found")?;
        match expr {
            Expr::Const(n) => Ok(*n),
            Expr::BinaryOp(var1, op, var2) => match op {
                Op::Plus => Ok(self.evaluate(var1)? + self.evaluate(var2)?),
                Op::Minus => Ok(self.evaluate(var1)? - self.evaluate(var2)?),
                Op::Mult => Ok(self.evaluate(var1)? * self.evaluate(var2)?),
                Op::Div => Ok(self.evaluate(var1)? / self.evaluate(var2)?),
                Op::Equal => Err("should not eval equal".to_string()),
            },
        }
    }

    fn find_humn_value(&self, from: &String) -> usize {
        let looking_for: String = "humn".to_string();
        let root_expr = self.vars.get(from).expect("hmm, root not found");
        // try both sides of this, recursing down the side that errors
        if let Expr::BinaryOp(var1, Op::Equal, var2) = root_expr {
            let left_result = self.evaluate(var1);
            let right_result = self.evaluate(var2);
            match (left_result, right_result) {
                (Ok(left_value), Err(_)) => self.find_missing_value(looking_for, var2, left_value),
                (Err(_), Ok(right_value)) => {
                    self.find_missing_value(looking_for, var1, right_value)
                }
                _ => panic!("One of these sides should error!"),
            }
        } else {
            panic!("oops, I setup root incorrectly");
        }
    }

    fn find_missing_value(&self, name: String, var: &String, target: usize) -> usize {
        println!("Looking for {}, target {}", var, target);
        if &name == var {
            return target;
        }
        let expr = self.vars.get(var).expect("var should be found for this");
        // try both sides of this, recursing down the side that errors
        match expr {
            Expr::Const(n) => *n,
            Expr::BinaryOp(var1, op, var2) => {
                let left_result = self.evaluate(var1);
                let right_result = self.evaluate(var2);
                match (left_result, right_result) {
                    (Ok(left_value), Err(_)) => match op {
                        // LV + x = T, x = T - LV
                        Op::Plus => self.find_missing_value(name, var2, target - left_value),
                        // LV - x = T, x = LV - T
                        Op::Minus => self.find_missing_value(name, var2, left_value - target),
                        // LV * x = T, x = T / LV
                        Op::Mult => self.find_missing_value(name, var2, target / left_value),
                        // LV / x = T, x = LV / T
                        Op::Div => self.find_missing_value(name, var2, left_value / target),
                        Op::Equal => panic!("only root should have this!"),
                    },
                    (Err(_), Ok(right_value)) => match op {
                        // x + RV = T, x = T - RV
                        Op::Plus => self.find_missing_value(name, var1, target - right_value),
                        // x - RV = T, x = T + RV
                        Op::Minus => self.find_missing_value(name, var1, target + right_value),
                        // x * RV = T, x = T / RV
                        Op::Mult => self.find_missing_value(name, var1, target / right_value),
                        // x / RV = T, x = T * RV
                        Op::Div => self.find_missing_value(name, var1, target * right_value),
                        Op::Equal => panic!("only root should have this!"),
                    },
                    _ => panic!("One of these sides should error!"),
                }
            }
        }
    }
}

pub fn part1(file_contents: String, _p: Option<Params>) -> String {
    let statements = parse_lines(&file_contents);
    let mut program = Program::new();
    program.load_statements(statements);
    let result = program
        .evaluate(&"root".to_string())
        .expect("this part shouldn't error");

    format!("{}", result)
}

pub fn part2(file_contents: String, _p: Option<Params>) -> String {
    let statements = parse_lines(&file_contents);
    let mut program = Program::new();
    program.load_statements_2(statements);
    let result = program.find_humn_value(&"root".to_string());

    format!("{}", result)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use crate::days::read_input_file;

    #[test]
    fn part1_example() {
        let input = read_input_file("inputs/day21-example.txt");
        assert_eq!(part1(input, None), "152".to_string());
    }

    #[test]
    fn part1_input() {
        let input = read_input_file("inputs/day21-input.txt");
        assert_eq!(part1(input, None), "38731621732448".to_string());
    }

    #[test]
    fn part2_example() {
        let input = read_input_file("inputs/day21-example.txt");
        assert_eq!(part2(input, None), "301".to_string());
    }

    #[test]
    fn part2_input() {
        let input = read_input_file("inputs/day21-input.txt");
        assert_eq!(part2(input, None), "3848301405790".to_string());
    }
}
