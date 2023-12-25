use std::fs;
use std::collections::{HashMap, HashSet};

struct Wire {
	from: String,
	to: HashSet<String>,
	used: bool,
}

fn parse_wire(line: &str) -> (&str, Wire) {
	let from_to: Vec<&str> = line.split(": ").collect();
	let mut wire = Wire {from: from_to[0].to_string(), to: HashSet::new(), used: true};
	let tos: Vec<&str> = from_to[1].split(' ').collect();
	for t in tos {
		wire.to.insert(t.to_string());
	}
	(from_to[0], wire)
}

fn read_file(filepath: &str) -> HashMap<String, Wire> {
	let contents = fs::read_to_string(filepath);
	let mut hash_map: HashMap<String, Wire> = HashMap::new();
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		if line != "" {
			let (name, wire) = parse_wire(line);
			hash_map.insert(name.to_string(), wire);
		}
	}
	hash_map
}

fn disconnect(hash_map: &mut HashMap<String, Wire>) -> usize {
	let mut total: usize = 0;
	for h1 in &mut *hash_map {
		for h2 in &mut *hash_map {
			for h3 in &mut *hash_map {
				if h1.0 != h2.0 || h1.0 != h3.0 || h2.0 != h3.0 {
					h1.1.used = false;
					h2.1.used = false;
					h3.1.used = false;
					// Count components.
					h1.1.used = true;
					h2.1.used = true;
					h3.1.used = true;
				}
			}
		}
	}
	total
}

fn part1() {
	let mut hash_map = read_file("INPUT");
	disconnect(&mut hash_map);
	println!("Part 1: {}", 0);
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 25 - Snowverload");
	part1();
	part2();
}
