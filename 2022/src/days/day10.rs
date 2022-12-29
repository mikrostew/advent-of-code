use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;

use super::parse_i32;
use run_aoc::runner_fn;

#[derive(Debug)]
enum Instruction {
    AddX(i32),
    Noop,
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    alt((addx, noop))(input)
}

fn addx(input: &str) -> IResult<&str, Instruction> {
    separated_pair(tag("addx"), tag(" "), parse_i32)(input)
        .map(|(next_input, (_addx, num))| (next_input, Instruction::AddX(num)))
}

fn noop(input: &str) -> IResult<&str, Instruction> {
    tag("noop")(input).map(|(next_input, _noop)| (next_input, Instruction::Noop))
}

struct SimpleCPU {
    x_register: i32,
}

impl SimpleCPU {
    fn new() -> Self {
        SimpleCPU { x_register: 1 }
    }

    // produces a Vec containing all x-register values at each cycle
    fn run_program(&mut self, p: &Vec<Instruction>) -> Vec<i32> {
        let mut x_reg_values = vec![];
        // push initial value, so indices are 1-based
        x_reg_values.push(self.x_register);

        for instr in p.iter() {
            match instr {
                Instruction::AddX(val) => {
                    // 2 cycles
                    x_reg_values.push(self.x_register);
                    x_reg_values.push(self.x_register);
                    self.x_register += val;
                }
                Instruction::Noop => {
                    // 1 cycle
                    x_reg_values.push(self.x_register);
                }
            }
        }
        x_reg_values
    }

    // produces a printout based on cycle and x-reg value
    fn run_program_with_crt(&mut self, p: &Vec<Instruction>) -> String {
        let mut cycle_value = 0;
        let mut pixels: Vec<char> = vec![];

        // TODO: still need to push initial value, so indices are 1-based?
        //x_reg_values.push(self.x_register);

        for instr in p.iter() {
            match instr {
                Instruction::AddX(val) => {
                    // 2 cycles
                    pixels.push(self.lit_or_dark(cycle_value));
                    cycle_value += 1;
                    pixels.push(self.lit_or_dark(cycle_value));
                    cycle_value += 1;
                    self.x_register += val;
                }
                Instruction::Noop => {
                    // 1 cycle
                    pixels.push(self.lit_or_dark(cycle_value));
                    cycle_value += 1;
                }
            }
        }
        pixels
            .chunks(40)
            .map(|line| line.iter().collect())
            .collect::<Vec<String>>()
            .join("\n")
    }

    // figure out what pixel should be drawn
    fn lit_or_dark(&self, cycle: i32) -> char {
        let line_pos = cycle % 40;
        if self.x_register - 1 <= line_pos && line_pos <= self.x_register + 1 {
            '#'
        } else {
            '.'
        }
    }
}

#[runner_fn]
fn part1(file_contents: String) -> i32 {
    let instructions: Vec<Instruction> = file_contents
        .lines()
        .map(|l| {
            let (leftover, instr) = instruction(l).expect("Could not parse line");
            assert_eq!(leftover, "");
            instr
        })
        .collect();
    //println!("{:?}", instructions);
    let mut cpu = SimpleCPU::new();
    let x_reg_values = cpu.run_program(&instructions);

    println!("value at 20: {}", x_reg_values[20]);
    println!("value at 60: {}", x_reg_values[60]);
    println!("value at 100: {}", x_reg_values[100]);
    println!("value at 140: {}", x_reg_values[140]);
    println!("value at 180: {}", x_reg_values[180]);
    println!("value at 220: {}", x_reg_values[220]);

    let signal_strength_sum = 20 * x_reg_values[20]
        + 60 * x_reg_values[60]
        + 100 * x_reg_values[100]
        + 140 * x_reg_values[140]
        + 180 * x_reg_values[180]
        + 220 * x_reg_values[220];

    signal_strength_sum
}

#[runner_fn]
fn part2(file_contents: String) -> String {
    let instructions: Vec<Instruction> = file_contents
        .lines()
        .map(|l| {
            let (leftover, instr) = instruction(l).expect("Could not parse line");
            assert_eq!(leftover, "");
            instr
        })
        .collect();
    //println!("{:?}", instructions);
    let mut cpu = SimpleCPU::new();
    let crt_display = cpu.run_program_with_crt(&instructions);

    crt_display
}

#[cfg(test)]
mod tests {
    use run_aoc::test_fn;

    test_fn!(day10, part1, example, 13140);
    test_fn!(day10, part1, input, 14320);

    test_fn!(
        day10,
        part2,
        example,
        "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
    );

    test_fn!(
        day10,
        part2,
        input,
        "###...##..###..###..#..#..##..###....##.
#..#.#..#.#..#.#..#.#.#..#..#.#..#....#.
#..#.#....#..#.###..##...#..#.#..#....#.
###..#....###..#..#.#.#..####.###.....#.
#....#..#.#....#..#.#.#..#..#.#....#..#.
#.....##..#....###..#..#.#..#.#.....##.."
    );
}
