use std::fs;
use std::collections::HashSet;

type Pos = (i64, i64);
type Houses = HashSet<Pos>;

fn read_file(filepath: &str) -> String {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	binding.split('\n').next().unwrap().to_string()
}

fn found_houses(line: &str) -> usize {
	let mut houses: Houses = HashSet::new();
	let mut pos: Pos = (0,0);
	houses.insert(pos);
	for c in line.chars() {
		match c {
			'>' => pos = (pos.0, pos.1 + 1),
			'<' => pos = (pos.0, pos.1 - 1),
			'v' => pos = (pos.0 + 1, pos.1),
			'^' => pos = (pos.0 - 1, pos.1),
			 _  => pos = pos,
		}
		houses.insert(pos);
	}
	houses.len()
}

fn robo_santa(line: &str) -> usize {
	let mut houses: Houses = HashSet::new();
	let mut pos: Pos = (0,0);
	let mut robo_pos: Pos = (0,0);
	houses.insert(pos);
	for c in line.chars() {
		match c {
			'>' => pos = (pos.0, pos.1 + 1),
			'<' => pos = (pos.0, pos.1 - 1),
			'v' => pos = (pos.0 + 1, pos.1),
			'^' => pos = (pos.0 - 1, pos.1),
			 _  => pos = pos,
		}
		houses.insert(pos);
		(pos, robo_pos) = (robo_pos, pos);
	}
	houses.len()
}


fn part1() {
	let line = read_file("INPUT");
	println!("Part 1: {}", found_houses(&line));
}

fn part2() {
	let line = read_file("INPUT");
	println!("Part 2: {}", robo_santa(&line));
}

fn main() {
	println!("Year 2015 day 3 - Perfectly Spherical Houses in a Vacuum");
	part1();
	part2();
}
