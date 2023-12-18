use std::fs;
use geo::{Coord, Polygon, Area, coord, LineString};

type Point<T> = (T,T);
type Instruction = (char, i32);

fn read_file(filepath: &str) -> (Vec<Instruction>, Vec<String>) {
	let mut instructions: Vec<Instruction> = vec!();
	let mut strings: Vec<String> = vec!();
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines {
		if line != "" {
			let mut parts = line.split(' ');
			let instruction = (parts.next().unwrap().chars().nth(0).unwrap(),
							   parts.next().unwrap().parse::<i32>().unwrap());
			instructions.push(instruction);
			strings.push(parts.next().unwrap().to_string());
		}
	}
	return (instructions, strings);
}

fn step(instruction: &Instruction, point: &Point<i32>) -> Point<i32> {
	match instruction.0 {
		'0' | 'R' => (point.0, point.1 + instruction.1),
		'1' | 'D' => (point.0 + instruction.1, point.1),
		'2' |'L' => (point.0, point.1 - instruction.1),
		'3' | 'U' => (point.0 - instruction.1, point.1),
		_   => (point.0, point.1),
	}
}

fn calculate_area(points: &Vec<Point<i32>>) -> f64 {
	let mut coordinates: Vec<Coord> = vec!();
	for point in points {
		coordinates.push(coord! {x: point.0 as f64, y: point.1 as f64 });
	}
	let line_string = LineString::new(coordinates);
	let polygon = Polygon::new(line_string, Vec::new());
	polygon.unsigned_area()
}

fn compute_from_instructions(instructions: &Vec<Instruction>) -> i64 {
	let mut points: Vec<Point<i32>> = vec!();
	let mut point: Point<i32> = (0,0);
	let mut b: i64 = 0;
	for instruction in instructions {
		points.push(point);
		point = step(instruction, &mut point);
		b += instruction.1 as i64;
	}
	let area = calculate_area(&points) as i64;
	2 + b + (area - b/2 - 1)
}

fn parse_string(string: &str) -> Instruction {
	let len: usize = string.len() - 2;
	let hex_string = &string[2 .. len];
	let num = i32::from_str_radix(&hex_string, 16).unwrap();
	(string.chars().nth(len).unwrap(), num)
}

fn part1(){
	let (instructions, _) = read_file("INPUT");
	println!("Part 1: {}", compute_from_instructions(&instructions));
}

fn part2(){
	let (_, strings) = read_file("INPUT");
	let instructions: Vec<Instruction> = strings.into_iter()
	                                            .map(|string| { parse_string(&string) })
	                                            .collect();
	println!("Part 2: {}", compute_from_instructions(&instructions));
}

fn main() {
	println!("Year 2023 day 18 - Lavaduct Lagoon");
	part1();
	part2();
}
