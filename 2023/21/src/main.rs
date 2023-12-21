use std::fs;
use std::collections::HashSet;

type Grid<T> = Vec<Vec<T>>;
type Position = (usize, usize);

fn read_file(filepath: &str) -> (Grid<bool>, Position) {
	let contents = fs::read_to_string(filepath);
	let mut garden: Grid<bool> = vec![];
	let mut start: Position = (0,0);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	let mut i: usize = 0;
	for line in lines{
		let mut garden_line: Vec<bool> = vec!();
		let mut j: usize = 0;
		for c in line.chars(){
			garden_line.push(c != '#');
			if c == 'S' {
				start = (i,j);
			}
			j += 1;
		}
		if garden_line.len() > 0 {
			garden.push(garden_line);
		}
		i += 1;
	}
	(garden, start)
}

fn add(a: usize, b: i8) -> usize {
	(a as i64 + b as i64) as usize
}

fn step(garden: &Grid<bool>, position: Position) -> Vec<Position> {
	let mut positions: Vec<Position> = vec!();
	let neighbors: [(i8, i8); 4] = [(1, 0), (-1,0), (0,1), (0,-1)];
	for n in neighbors {
		let pos = (add(position.0,n.0), add(position.1,n.1));
		if pos.0 >= garden.len() || pos.1 >= garden[pos.0].len() {
			continue;
		}
		if garden[pos.0][pos.1] {
			positions.push(pos);
		}
	}
	positions
}

fn steps(garden: &Grid<bool>, positions: &mut HashSet<Position>) {
	let pos: Vec<Position> = positions.clone().into_iter().collect();
	positions.clear();
	for p in pos {
		let neighbors = step(garden, p);
		for n in neighbors {
			positions.insert(n);
		}
	}
}

fn part1() {
	let (garden, start) = read_file("INPUT");
	let mut set: HashSet<Position> = HashSet::new();
	set.insert(start);
	for _ in 0 .. 64 {
		steps(&garden, &mut set);
	}
	println!("Part 1: {}", set.len());
}

fn part2() {
	// 26501365 steps and infinite repetition of the garden
	let (_garden, _start) = read_file("INPUT");
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 21 - Step Counter");
	part1();
	part2();
}
