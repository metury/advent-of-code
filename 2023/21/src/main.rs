use std::fs;
use std::collections::HashSet;

type Grid<T> = Vec<Vec<T>>;
type Position = (usize, usize);
type ModPos = (i64, i64);

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

fn add_mod(a: i64, b: i8) -> i64 {
	a + b as i64
}

// This is actually wrong. Because if it is less than zero we have to subtract it.
fn mod_in_garden(garden: &Grid<bool>, x: i64, y: i64) -> bool {
	let real_x: usize = (x % garden.len() as i64).abs() as usize;
	let real_y: usize = (y % garden[real_x].len() as i64).abs() as usize;
	garden[real_x][real_y]
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

fn step_mod(garden: &Grid<bool>, position: ModPos) -> Vec<ModPos> {
	let mut positions: Vec<ModPos> = vec!();
	let neighbors: [(i8, i8); 4] = [(1, 0), (-1,0), (0,1), (0,-1)];
	for n in neighbors {
		let pos = (add_mod(position.0,n.0), add_mod(position.1,n.1));
		if mod_in_garden(garden, pos.0, pos.1) {
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

fn odd_even_steps(garden: &Grid<bool>, positions: &mut HashSet<ModPos>, odd: &mut HashSet<ModPos>, even: &mut HashSet<ModPos>, parity: bool) {
	let pos: Vec<ModPos> = positions.clone().into_iter().collect();
	positions.clear();
	for p in pos {
		let neighbors = step_mod(garden, p);
		for n in neighbors {
			if odd.contains(&n) || even.contains(&n) {
				continue;
			}
			if parity {
				even.insert(n);
			}
			else {
				odd.insert(n);
			}
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
	let (garden, start) = read_file("INPUT");
	let mut set: HashSet<ModPos> = HashSet::new();
	let mut odd: HashSet<ModPos> = HashSet::new();
	let mut even: HashSet<ModPos> = HashSet::new();
	set.insert((start.0 as i64, start.1 as i64));
	for i in 0 .. 10 {
		odd_even_steps(&garden, &mut set, &mut odd, &mut even, i % 2 == 0);
	}
	println!("Part 2: {}", odd.len());
}

fn main() {
	println!("Year 2023 day 21 - Step Counter");
	part1();
	part2();
}
