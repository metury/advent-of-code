use regex::Regex;
use std::fs;

type Grid<T> = Vec<Vec<T>>;
type Pos = (usize, usize);

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
enum InstructionType {
    TurnOff,
    TurnOn,
    Toggle,
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Instruction {
    start: Pos,
    end: Pos,
    instr_type: InstructionType,
}

fn read_file(filepath: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(filepath);
    let binding = contents.expect("REASON");
    let mut instructions: Vec<Instruction> = vec![];
    let input =
        Regex::new(r"(turn off|turn on|toggle) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)")
            .unwrap();
    for (_, [ty, s1, s2, e1, e2]) in input.captures_iter(&binding).map(|c| c.extract()) {
        let instr_type: InstructionType;
        if ty == "turn off" {
            instr_type = InstructionType::TurnOff;
        } else if ty == "turn on" {
            instr_type = InstructionType::TurnOn;
        } else {
            instr_type = InstructionType::Toggle;
        }
        instructions.push(Instruction {
            instr_type: instr_type,
            start: (s1.parse().unwrap(), s2.parse().unwrap()),
            end: (e1.parse().unwrap(), e2.parse().unwrap()),
        });
    }
    instructions
}

fn part1() {
    let instructions = read_file("INPUT");
    let mut grid: Grid<bool> = vec![vec!(false; 1000); 1000];
    for ins in instructions {
        match ins.instr_type {
            InstructionType::Toggle => {
                for i in ins.start.0..ins.end.0 + 1 {
                    for j in ins.start.1..ins.end.1 + 1 {
                        grid[i][j] = !grid[i][j];
                    }
                }
            }
            InstructionType::TurnOff => {
                for i in ins.start.0..ins.end.0 + 1 {
                    for j in ins.start.1..ins.end.1 + 1 {
                        grid[i][j] = false;
                    }
                }
            }
            InstructionType::TurnOn => {
                for i in ins.start.0..ins.end.0 + 1 {
                    for j in ins.start.1..ins.end.1 + 1 {
                        grid[i][j] = true;
                    }
                }
            }
        }
    }
    let lit_up: u64 = grid
        .into_iter()
        .map(|vec| vec.into_iter().map(|x| if x { 1 } else { 0 }).sum::<u64>())
        .sum();
    println!("Part 1: {}", lit_up);
}

fn part2() {
    let instructions = read_file("INPUT");
    let mut grid: Grid<u64> = vec![vec!(0_u64; 1000); 1000];
    for ins in instructions {
        match ins.instr_type {
            InstructionType::Toggle => {
                for i in ins.start.0..ins.end.0 + 1 {
                    for j in ins.start.1..ins.end.1 + 1 {
                        grid[i][j] += 2;
                    }
                }
            }
            InstructionType::TurnOff => {
                for i in ins.start.0..ins.end.0 + 1 {
                    for j in ins.start.1..ins.end.1 + 1 {
                        grid[i][j] = if grid[i][j] > 0 {
                            grid[i][j] - 1
                        } else {
                            grid[i][j]
                        };
                    }
                }
            }
            InstructionType::TurnOn => {
                for i in ins.start.0..ins.end.0 + 1 {
                    for j in ins.start.1..ins.end.1 + 1 {
                        grid[i][j] += 1;
                    }
                }
            }
        }
    }
    let lit_up: u64 = grid
        .into_iter()
        .map(|vec| vec.into_iter().sum::<u64>())
        .sum();
    println!("Part 2: {}", lit_up);
}

fn main() {
    println!("Year 2015 day 6 - Probably a Fire Hazard");
    part1();
    part2();
}
