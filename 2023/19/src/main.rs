use std::fs;
use std::collections::HashMap;
use std::cmp::{min, max};

const X: usize = 0;
const M: usize = 1;
const A: usize = 2;
const S: usize = 3;
const INV: usize = 5;

type Part = [i64; 4];

type BoundPart = [(i64,i64); 4];

enum Compare {
	LessThan,
	MoreThan,
	True,
}

struct Rule {
	considering: usize,
	compare: Compare,
	number: i64,
	next_state: String,
	accepting: bool,
	rejecting: bool,
}

type State = Vec<Rule>;

fn get_considering(c: char) -> usize {
	match c {
		'x' => return X,
		'm' => return M,
		'a' => return A,
		's' => return S,
		 _  => return INV,
	 }
 }

fn parse_rule(line: &str) -> Rule {
	let splitted = line.split(':');
	let parts: Vec<&str> = splitted.collect();
	if parts.len() == 1 as usize {
		let new_state = parts[0];
		if new_state == "A" {
			return Rule {considering: INV,
				compare: Compare::True,
				number: 0,
				next_state: "".to_string(),
				accepting: true,
				rejecting: false};
		}
		else if new_state == "R" {
			return Rule {considering: INV,
				compare: Compare::True,
				number: 0,
				next_state: "".to_string(),
				accepting: false,
				rejecting: true};
		}
		else {
			return Rule {considering: INV,
				compare: Compare::True,
				number: 0,
				next_state: new_state.to_string(),
				accepting: false,
				rejecting: false};
		}
	}
	let binding = parts[0].to_string();
	let considering = get_considering(binding.chars().nth(0).unwrap());
	let compare: Compare;
	match binding.chars().nth(1).unwrap() {
		'<' => compare = Compare::LessThan,
		'>' => compare = Compare::MoreThan,
		_   => compare = Compare::True,
	}
	let number = i64::from_str_radix(&binding[2 ..], 10).unwrap();
	let new_state = parts[1].to_string();
	if new_state == "A" {
		return Rule {considering: considering,
			compare: compare,
			number: number,
			next_state: "".to_string(),
			accepting: true,
			rejecting: false};
	}
	else if new_state == "R" {
		return Rule {considering: considering,
			compare: compare,
			number: number,
			next_state: "".to_string(),
			accepting: false,
			rejecting: true};
	}
	else {
		return Rule {considering: considering,
			compare: compare,
			number: number,
			next_state: new_state,
			accepting: false,
			rejecting: false};
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
	(string, rules)
}

fn parse_part(line: &str) -> Part {
	let interior = &line[1 .. line.len() - 1];
	let values: Vec<&str> = interior.split(',').collect();
	let mut part: Part = [0; 4];
	for val in values {
		match val.chars().nth(0).unwrap() {
			'x' => part[X] = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			'm' => part[M] = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			'a' => part[A] = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			's' => part[S] = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			 _  => todo!(),
		}
	}
	part
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
			let (string, state) = parse_state(&line);
			hash_map.insert(string, state);
		}
		else {
			parts.push(parse_part(line));
		}
	}
	(hash_map, parts)
}

fn process_part(hash_map: &HashMap<String, State>, part: Part) -> i64 {
	let mut string = "in";
	let ret: i64 = part[X] + part[M] + part[A] + part[S];
	loop {
		let state = &hash_map[string];
		for rule in state {
			match rule.compare {
				Compare::LessThan => {
					if part[rule.considering] < rule.number {
						if rule.accepting {
							return ret;
						}
						else if rule.rejecting {
							return 0;
						}
						string = &rule.next_state;
						break;
					}
				}
				Compare::MoreThan => {
					if part[rule.considering] > rule.number {
						if rule.accepting {
							return ret;
						}
						else if rule.rejecting {
							return 0;
						}
						string = &rule.next_state;
						break;
					}
				}
				Compare::True => {
					if rule.accepting {
						return ret;
					}
					else if rule.rejecting {
						return 0;
					}
					string = &rule.next_state;
					break;
				}
			}
		}
	}
}

fn check_bound(bound: (i64, i64)) -> bool {
	bound.1 > bound.0
}

fn count_possibilites(bounds: &BoundPart) -> i64 {
	let res = (bounds[X].1 - bounds[X].0) * (bounds[M].1 - bounds[M].0) * (bounds[A].1 - bounds[A].0) * (bounds[S].1 - bounds[S].0);
	res
}

fn process_bounds(hash_map: &HashMap<String, State>, bounds: &mut BoundPart, string: &str) -> i64 {
	let mut total: i64 = 0;
	let state = &hash_map[string];
	for rule in state {
		match rule.compare {
			Compare::LessThan => {
				let part1 = (bounds[rule.considering].0, rule.number);
				let mut new_bounds = [bounds[X], bounds[M], bounds[A], bounds[S]];
				bounds[rule.considering] = (max(rule.number, bounds[rule.considering].0), bounds[rule.considering].1);
				new_bounds[rule.considering] = part1;
				if check_bound(part1){
					if rule.accepting {
						total += count_possibilites(&new_bounds);
					}
					else if rule.rejecting {
						total += 0;
					}
					else {
						total += process_bounds(hash_map, &mut new_bounds, &rule.next_state);
					}
				}
			}
			Compare::MoreThan => {
				let part1 = (rule.number + 1, bounds[rule.considering].1);
				let mut new_bounds = [bounds[X], bounds[M], bounds[A], bounds[S]];
				bounds[rule.considering] = (bounds[rule.considering].0, min(rule.number + 1, bounds[rule.considering].1));
				new_bounds[rule.considering] = part1;
				if check_bound(part1){
					if rule.accepting {
						total += count_possibilites(&new_bounds);
					}
					else if rule.rejecting {
						total += 0;
					}
					else {
						total += process_bounds(hash_map, &mut new_bounds, &rule.next_state);
					}
				}
			}
			Compare::True => {
				let mut new_bounds = [bounds[X], bounds[M], bounds[A], bounds[S]];
				if rule.accepting {
					total += count_possibilites(&bounds);
				}
				else if rule.rejecting {
					total += 0;
				}
				else {
					total += process_bounds(hash_map, &mut new_bounds, &rule.next_state);
				}
			}
		}
	}
	total
}

fn part1() {
	let (hash_map, parts) = read_file("INPUT");
	let results: Vec<i64> = parts.into_iter().map(|p| process_part(&hash_map, p)).collect();
	println!("Part 1: {}", results.iter().sum::<i64>());
}

fn part2() {
	let (hash_map, _) = read_file("INPUT");
	let mut part: BoundPart = [(1, 4001); 4];
	println!("Part 2: {}", process_bounds(&hash_map, &mut part, "in"));
}

fn main() {
	println!("Year 2023 day 19 - Aplenty");
	part1();
	part2();
}
