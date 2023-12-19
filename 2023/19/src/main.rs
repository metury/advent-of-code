use std::fs;
use std::collections::HashMap;

struct Part {
	x: i64,
	m: i64,
	a: i64,
	s: i64,
}

enum Compare {
	LESS_THAN,
	MORE_THAN,
}

struct Rule {
	considering: char,
	compare: Compare,
	next_state: String,
	accepting: bool,
	rejecting: bool,
}

fn read_file(filepath: &str) -> (HashMap<String, Rule>, Vec<Part>) {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		for c in line.chars(){
		}
	}
	return;
}

fn part1() {
	println!("Part 1: {}", 0);
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 19 - Aplenty");
	part1();
	part2();
}
