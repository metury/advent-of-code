use std::fs;
use std::collections::HashMap;

struct Part {
	x: i64,
	m: i64,
	a: i64,
	s: i64,
}

#[derive(Debug)]
enum Compare {
	LessThan,
	MoreThan,
	True,
}

#[derive(Debug)]
struct Rule {
	considering: char,
	compare: Compare,
	number: i64,
	next_state: String,
	accepting: bool,
	rejecting: bool,
}

struct State {
	rules: Vec<Rule>,
}

fn parse_rule(line: &str) -> Rule {
	let splitted = line.split(':');
	let parts: Vec<&str> = splitted.collect();
	if parts.len() == 1 as usize {
		let new_state = parts[0];
		if new_state == "A" {
			return Rule { considering: '@', compare: Compare::True, number: 0,
	              next_state: "".to_string(), accepting: true, rejecting: false};
		}
		else if new_state == "R" {
			return Rule { considering: '@', compare: Compare::True, number: 0,
	              next_state: "".to_string(), accepting: false, rejecting: true};
		}
		else {
			return Rule { considering: '@', compare: Compare::True, number: 0,
	              next_state: new_state.to_string(), accepting: false, rejecting: false};
		}
	}
	let binding = parts[0].to_string();
	let considering = binding.chars().nth(0).unwrap();
	let mut compare: Compare;
	match binding.chars().nth(1).unwrap() {
		'<' => compare = Compare::LessThan,
		'>' => compare = Compare::MoreThan,
		_   => compare = Compare::True,
	}
	let number = i64::from_str_radix(&binding[2 ..], 10).unwrap();
	let new_state = parts[1].to_string();
	if new_state == "A" {
		return Rule { considering: considering, compare: compare, number: number,
	              next_state: "".to_string(), accepting: true, rejecting: false};
	}
	else if new_state == "R" {
		return Rule { considering: considering, compare: compare, number: number,
	              next_state: "".to_string(), accepting: false, rejecting: true};
	}
	else {
		return Rule { considering: considering, compare: compare, number: number,
	              next_state: new_state, accepting: false, rejecting: false};
	}
}

fn parse_state(line: &str) -> (String, State) {
	let mut parts = line.split('{');
	let string = parts.next().unwrap().to_string();
	let mut rules: Vec<Rule> = vec!();
	let binding = parts.next().unwrap().to_string();
	let rule = &binding[0 .. binding.len() - 1];
	for r in rule.split(',') {
		rules.push(parse_rule(r));
	}
	println!("String {} and rules {:?}", string, rules);
	(string, State{rules: rules})
}

fn read_file(filepath: &str) -> (HashMap<String, State>, Vec<Part>) {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	let mut parts: Vec<Part> = vec!();
	let mut hash_map: HashMap<String, State> = HashMap::new();
	let mut first_part: bool = true;
	for line in lines{
		if line == "" {
			first_part = false;
		}
		else if first_part {
			parse_state(&line);
		}
		else {
			// Parse Part.
		}
	}
	(hash_map, parts)
}

fn part1() {
	read_file("INPUT");
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
