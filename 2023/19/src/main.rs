use std::fs;
use std::collections::HashMap;

struct Part {
	x: i64,
	m: i64,
	a: i64,
	s: i64,
}

struct BoundPart {
	x_l: i64,
	x_h: i64,
	m_l: i64,
	m_h: i64,
	a_l: i64,
	a_h: i64,
	s_l: i64,
	s_h: i64,
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

type State = Vec<Rule>;

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
	let compare: Compare;
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
	(string, rules)
}

fn parse_part(line: &str) -> Part {
	let interior = &line[1 .. line.len() - 1];
	let values: Vec<&str> = interior.split(',').collect();
	let mut x: i64 = 0;
	let mut a: i64 = 0;
	let mut s: i64 = 0;
	let mut m: i64 = 0;
	for val in values {
		match val.chars().nth(0).unwrap() {
			'x' => x = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			'a' => a = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			's' => s = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			'm' => m = i64::from_str_radix(&val[2 ..], 10).unwrap(),
			 _  => todo!(),
		 }
	 }
	 Part {x: x, a: a, s: s, m: m}
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
	let ret: i64 = part.x + part.a + part.s + part.m;
	loop {
		let state = &hash_map[string];
		for rule in state {
			match rule.compare {
				Compare::LessThan => {
					match rule.considering {
						'x' => {
							if part.x < rule.number {
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
						'a' => {
							if part.a < rule.number {
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
						's' => {
							if part.s < rule.number {
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
						'm' => {
							if part.m < rule.number {
								if rule.accepting {
									return ret;
								}
								else if rule.rejecting {
									return 0;
								}
								string = &rule.next_state;
								break;
							}
						},
						_ => todo!(),
					}
				}
				Compare::MoreThan => {
					match rule.considering {
						'x' => {
							if part.x > rule.number {
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
						'a' => {
							if part.a > rule.number {
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
						's' => {
							if part.s > rule.number {
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
						'm' => {
							if part.m > rule.number {
								if rule.accepting {
									return ret;
								}
								else if rule.rejecting {
									return 0;
								}
								string = &rule.next_state;
								break;
							}
						},
						_ => todo!(),
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

fn process_bounds(hash_map: &HashMap<String, State>, bounds: BoundPart) -> i64 {
	return 0;
}

fn part1() {
	let (hash_map, parts) = read_file("INPUT");
	let results: Vec<i64> = parts.into_iter().map(|p| process_part(&hash_map, p)).collect();
	println!("Part 1: {}", results.iter().sum::<i64>());
}

fn part2() {
	let (hash_map, _) = read_file("INPUT");
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 19 - Aplenty");
	part1();
	part2();
}
