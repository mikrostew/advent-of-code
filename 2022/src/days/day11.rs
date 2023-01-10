use std::collections::VecDeque;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::complete::newline;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

use run_aoc::runner_fn;
use utils::nom_usize;
use utils::traits::ToNum;

struct Monkey<'a> {
    number: usize,
    items: VecDeque<usize>,
    operation_fn: Box<dyn Fn(usize) -> usize + 'a>,
    test_fn: Box<dyn Fn(usize) -> usize + 'a>,
    num_inspections: usize,
    divis_by: usize,
}

enum Operator {
    Add,
    Multiply,
}

enum Operand {
    Old,
    Num(usize),
}

fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(newline, monkey)(input)
}

fn monkey(input: &str) -> IResult<&str, Monkey> {
    map(
        tuple((monkey_number, starting_items, operation, test)),
        |(number, items, (operator, operand), (divis_by, true_monkey, false_monkey))| {
            let operator_fn = move |a, b| match operator {
                Operator::Add => a + b,
                Operator::Multiply => a * b,
            };
            Monkey {
                number,
                items: VecDeque::from(items),
                operation_fn: Box::new(move |x| match operand {
                    Operand::Num(n) => operator_fn(x, n),
                    Operand::Old => operator_fn(x, x),
                }),
                test_fn: Box::new(move |x| {
                    if x % divis_by == 0 {
                        true_monkey
                    } else {
                        false_monkey
                    }
                }),
                num_inspections: 0,
                divis_by,
            }
        },
    )(input)
}

fn monkey_number(input: &str) -> IResult<&str, usize> {
    delimited(tag("Monkey "), nom_usize, tuple((tag(":"), newline)))(input)
}

fn starting_items(input: &str) -> IResult<&str, Vec<usize>> {
    delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), nom_usize),
        newline,
    )(input)
}

fn operation(input: &str) -> IResult<&str, (Operator, Operand)> {
    delimited(
        tag("  Operation: new = old"),
        tuple((operator, operand)),
        newline,
    )(input)
}

fn operator(input: &str) -> IResult<&str, Operator> {
    map(delimited(tag(" "), one_of("*+"), tag(" ")), |op| match op {
        '*' => Operator::Multiply,
        '+' => Operator::Add,
        _ => unreachable!(),
    })(input)
}

// returns the number of identifier used as the operand
fn operand(input: &str) -> IResult<&str, Operand> {
    map(alt((tag("old"), digit1)), |num_or_old| match num_or_old {
        "old" => Operand::Old,
        _ => Operand::Num(num_or_old.to_usize()),
    })(input)
}

// returns three values used in this test
fn test(input: &str) -> IResult<&str, (usize, usize, usize)> {
    tuple((test_cond, if_true, if_false))(input)
}

fn test_cond(input: &str) -> IResult<&str, usize> {
    delimited(tag("  Test: divisible by "), nom_usize, newline)(input)
}

fn if_true(input: &str) -> IResult<&str, usize> {
    delimited(tag("    If true: throw to monkey "), nom_usize, newline)(input)
}

fn if_false(input: &str) -> IResult<&str, usize> {
    delimited(tag("    If false: throw to monkey "), nom_usize, newline)(input)
}

fn do_round1(monkeys: &mut Vec<Monkey>) -> () {
    for m in 0..monkeys.len() {
        //println!("Monkey {}", m);
        while let Some(level) = monkeys[m].items.pop_front() {
            monkeys[m].num_inspections += 1;
            //println!("inspect item level {}", level);
            let op_level = (monkeys[m].operation_fn)(level);
            //println!("worry level --> {}", op_level);
            let bored_level = op_level / 3;
            //println!("bored level --> {}", bored_level);
            let to_monkey = (monkeys[m].test_fn)(bored_level);
            //println!("thrown to monkey {}", to_monkey);
            monkeys[to_monkey].items.push_back(bored_level);
        }
    }
}

// replace the division by 3 with modulo the LCM of the monkeys to prevent overflow
fn do_round2(monkeys: &mut Vec<Monkey>, lcm: usize) -> () {
    for m in 0..monkeys.len() {
        while let Some(level) = monkeys[m].items.pop_front() {
            monkeys[m].num_inspections += 1;
            let op_level = (monkeys[m].operation_fn)(level);
            let bored_level = op_level % lcm;
            let to_monkey = (monkeys[m].test_fn)(bored_level);
            monkeys[to_monkey].items.push_back(bored_level);
        }
    }
}

#[runner_fn]
fn part1(file_contents: String) -> usize {
    //println!("{}", file_contents);
    let (leftover, mut monkeys) =
        parse_monkeys(&file_contents).expect("Could not parse monkeys from input!");
    assert_eq!(leftover, "");
    //println!("num monkeys: {}", monkeys.len());
    // check monkey order
    monkeys
        .iter()
        .enumerate()
        .for_each(|(i, m)| assert_eq!(m.number, i));

    for _ in 0..20 {
        do_round1(&mut monkeys);
    }

    // calculate the top 2 by # of inspections
    let mut inspections = monkeys
        .iter()
        .map(|m| m.num_inspections)
        .collect::<Vec<usize>>();
    inspections.sort();
    inspections.reverse();
    let top_2_product = inspections[0..2].iter().product::<usize>();

    top_2_product
}

#[runner_fn]
fn part2(file_contents: String) -> usize {
    let (leftover, mut monkeys) =
        parse_monkeys(&file_contents).expect("Could not parse monkeys from input!");
    assert_eq!(leftover, "");
    // check monkey order
    monkeys
        .iter()
        .enumerate()
        .for_each(|(i, m)| assert_eq!(m.number, i));

    // calculate the least common multiple
    // (all divisors are different primes)
    let lcm = monkeys
        .iter()
        .map(|m| m.divis_by)
        .collect::<Vec<usize>>()
        .iter()
        .product();

    for _ in 0..10000 {
        do_round2(&mut monkeys, lcm);
    }

    // calculate the top 2 by # of inspections
    let mut inspections = monkeys
        .iter()
        .map(|m| m.num_inspections)
        .collect::<Vec<usize>>();
    println!("inspections: {:?}", inspections);

    inspections.sort();
    inspections.reverse();
    let top_2_product = inspections[0..2].iter().product::<usize>();

    top_2_product
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day11, part1, example, 10605);
    test_fn!(day11, part1, input, 67830);

    test_fn!(day11, part2, example, 2713310158usize);
    test_fn!(day11, part2, input, 15305381442usize);
}
