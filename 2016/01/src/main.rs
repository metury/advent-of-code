use std::fs;
use regex::Regex;
use std::collections::HashSet;

#[derive(Copy, Clone, Eq, Debug, PartialEq, Ord, PartialOrd)]
enum Rotation {
	R,
	L,
}

struct Movement {
	rot: Rotation,
	mult: i64
}

type Position = (i64, i64);

const NORTH: Position = (0, -1);
const SOUTH: Position = (0, 1);
const EAST: Position = (1, 0);
const WEST: Position = (-1, 0);

fn read_file(filepath: &str) -> Vec<Movement> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let mut movements: Vec<Movement> = vec!();
	let re = Regex::new(r"(R|L)([0-9]+)").unwrap();
	for(_, [rotation, movement]) in re.captures_iter(&binding).map(|c| c.extract()) {
		if rotation == "R" {
			movements.push(Movement{rot: Rotation::R, mult: movement.parse().unwrap()});
		} else {
			movements.push(Movement{rot: Rotation::L, mult: movement.parse().unwrap()});
		}
	}
	movements
}

fn get_shift(rot: Rotation, direction: Position) -> Position {
	match rot {
		Rotation::R => {
			if direction == NORTH {return EAST;}
			else if direction == EAST {return SOUTH;}
			else if direction == SOUTH {return WEST;}
			else {return NORTH;}
		},
		Rotation::L => {
			if direction == NORTH {return WEST;}
			else if direction == WEST {return SOUTH;}
			else if direction == SOUTH {return EAST;}
			else {return NORTH;}
		},
	}
}

fn distance(movements: &Vec<Movement>, stop: bool) -> i64 {
	let mut positions: HashSet<Position> = HashSet::new();
	let mut pos: Position = (0,0);
	let mut shift: Position = NORTH;
	'outer: for mov in movements {
		shift = get_shift(mov.rot, shift);
		for _ in 0 .. mov.mult {
			pos.0 += shift.0;
			pos.1 += shift.1;
			if positions.contains(&pos) && stop {
				break 'outer;
			}
			positions.insert(pos);
		}
	}
	pos.0.abs() + pos.1.abs()
}

fn part1() {
	let movements = read_file("INPUT");
	println!("Part 1: {}", distance(&movements, false));
}

fn part2() {
	let movements = read_file("INPUT");
	println!("Part 2: {}", distance(&movements, true));
}

fn main() {
	println!("Year 2016 day 1 - No Time for a Taxicab");
	part1();
	part2();
}
