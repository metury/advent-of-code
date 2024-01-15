use std::fs;
use regex::Regex;
use std::collections::HashMap;
use std::option::Option;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum GateType {
	Or,
	And,
	LShift,
	RShift,
	Not,
	Simple,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Gate {
	inputs: [String; 2],
	gate_type: GateType,
	output: String,
}

impl Gate {
	fn eval(&self, assigned: &mut HashMap<String, u16>) {
		let mut first: u16 = 0;
		let mut second: u16 = 0;
		match get_number(&self.inputs[0], assigned) {
			Some(nr) => first = nr,
			None => return,
		}
		match get_number(&self.inputs[1], assigned) {
			Some(nr) => second = nr,
			None => return,
		}
		match self.gate_type {
			GateType::Or => assigned.insert(self.output.clone(), first | second),
			GateType::And => assigned.insert(self.output.clone(), first & second),
			GateType::LShift => assigned.insert(self.output.clone(), first << second),
			GateType::RShift => assigned.insert(self.output.clone(), first >> second),
			GateType::Not => assigned.insert(self.output.clone(), !first),
			GateType::Simple => assigned.insert(self.output.clone(), first),
		};
	}
}

fn get_number(written: &str, assigned: &HashMap<String, u16>) -> Option<u16> {
	if assigned.contains_key(written){
		return Some(assigned[written]);
	} else if written.chars().nth(0)?.is_digit(10) {
		return Some(written.parse::<u16>().unwrap());
	}
	None
}

fn read_file(filepath: &str) -> Vec<Gate> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let mut gates: Vec<Gate> = vec!();
	let bin_re = Regex::new(r"([^\s]+) (OR|AND|LSHIFT|RSHIFT) ([^\s]+) -> (.+)").unwrap();
	let not_re = Regex::new(r"NOT ([^\s]+) -> (.+)").unwrap();
	let simple_re = Regex::new(r"\n([^\s]+) -> (.+)").unwrap();
	for (_, [first, oper, second, res]) in bin_re.captures_iter(&binding).map(|c| c.extract()) {
		let operator: GateType;
		match oper {
			"OR" => operator = GateType::Or,
			"AND" => operator = GateType::And,
			"LSHIFT" => operator = GateType::LShift,
			"RSHIFT" => operator = GateType::RShift,
			_ => todo!(),
		}
		gates.push(Gate {inputs: [first.to_string(), second.to_string()], gate_type: operator, output: res.to_string()});
	}
	for (_, [first, res]) in not_re.captures_iter(&binding).map(|c| c.extract()) {
		gates.push(Gate {inputs: [first.to_string(), "".to_string()], gate_type: GateType::Not, output: res.to_string()});
	}
	for (_, [first, res]) in simple_re.captures_iter(&binding).map(|c| c.extract()) {
		gates.push(Gate {inputs: [first.to_string(), "".to_string()], gate_type: GateType::Simple, output: res.to_string()});
	}
	gates
}



fn part1() {
	let gates = read_file("INPUT");
	let mut assigned: HashMap<String, u16> = HashMap::new();
	assigned.insert("".to_string(), 0);
	while !assigned.contains_key("a") {
		for gate in &gates {
			gate.eval(&mut assigned);
		}
	}
	println!("Part 1: {}", assigned["a"]);
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2015 day 7 - Some Assembly Required");
	part1();
	part2();
}
