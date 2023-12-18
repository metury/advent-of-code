use std::fs;
use std::collections::HashMap;

type Grid<T> = Vec<Vec<T>>;
type Point<T> = (T,T);
type Instruction = (char, i32, String);

fn read_file(filepath: &str) -> Vec<Instruction> {
	let mut instructions: Vec<Instruction> = vec!();
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines {
		if line != "" {
			let mut parts = line.split(' ');
			let instruction = (parts.next().unwrap().chars().nth(0).unwrap(),
							   parts.next().unwrap().parse::<i32>().unwrap(),
							   parts.next().unwrap().to_string());
			instructions.push(instruction);
		}
	}
	return instructions;
}

fn step(instruction: &Instruction, point: &Point<i32>) -> Point<i32> {
	match instruction.0 {
		'L' => (point.0, point.1 - instruction.1),
		'R' => (point.0, point.1 + instruction.1),
		'U' => (point.0 - instruction.1, point.1),
		'D' => (point.0 + instruction.1, point.1),
		_   => (point.0, point.1),
	}
}

fn outline(grid: &mut Grid<bool>, instructions: &Vec<Instruction>, point: &mut Point<i32>) {
	for instruction in instructions {
		grid[point.0 as usize][point.1 as usize] = true;
		let tmp = step(&instruction, &point);
		for i in point.0 .. tmp.0 {
			grid[i as usize][point.1 as usize] = true;
		}
		for i in tmp.0 .. point.0 {
			grid[i as usize][point.1 as usize] = true;
		}
		for i in point.1 .. tmp.1 {
			grid[point.0 as usize][i as usize] = true;
		}
		for i in tmp.1 .. point.1 {
			grid[point.0 as usize][i as usize] = true;
		}
		*point = (tmp.0, tmp.1);
	}
}

fn flood_fill(grid: &mut Grid<bool>, point: Point<usize>) {
	if point.0 >= grid.len() || point.1 >= grid[point.0].len() {
		return;
	}
	if grid[point.0][point.1] {
		return;
	}

	grid[point.0][point.1] = true;
	flood_fill(grid, (point.0 + 1, point.1));
	flood_fill(grid, (point.0 - 1, point.1));
	flood_fill(grid, (point.0, point.1 - 1));
	flood_fill(grid, (point.0, point.1 + 1));
}

fn compute_space(grid: &Grid<bool>) -> i64 {
	let mut total: i64 = 0;
	for line in grid {
		for cell in line {
			if *cell {
				total += 1;
			}
		}
	}
	total
}

fn print(grid: &Grid<bool>){
	for line in grid {
		for c in line {
			if *c {
				print!("#");
			}
			else {
				print!(".");
			}
		}
		println!();
	}
}

fn part1(){
	let instructions = read_file("INPUT");
	let mut point: Point<i32> = (0,0);
	let mut min = point;
	let mut max = point;
	for instruction in &instructions {
		point = step(&instruction, &mut point);
		min = (if point.0 < min.0 { point.0 } else { min.0 },
		       if point.1 < min.1 { point.1 } else { min.1 });
		max = (if point.0 > max.0 { point.0 } else { max.0 },
		       if point.1 > max.1 { point.1 } else { max.1 });
	}
	let mut grid: Grid<bool> = vec![vec![false; (max.1 - min.1 + 1) as usize]; (max.0 - min.0 + 1) as usize];
	outline(&mut grid, &instructions, &mut (-min.0, -min.1));
	flood_fill(&mut grid, (13, 180));
	println!("Part 1: {}", compute_space(&grid));
}

fn part2(){
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 18 - Lavaduct Lagoon");
	part1();
	part2();
}
