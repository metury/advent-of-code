use std::fs;
use std::io::Write;
use std::collections::{HashSet, VecDeque};

struct Wire {
	start: String,
	ends: Vec<String>,
}

fn parse_wire(line: &str) -> Wire {
	let from_to: Vec<&str> = line.split(": ").collect();
	let mut wire = Wire{start: from_to[0].to_string(), ends: vec!()};
	let parts: Vec<&str> = from_to[1].split(' ').collect();
	for p in parts {
		wire.ends.push(p.to_string());
	}
	wire
}

fn read_file(filepath: &str) -> Vec<Wire> {
	let contents = fs::read_to_string(filepath);
	let mut wires: Vec<Wire> = vec!();
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		if line != "" {
			wires.push(parse_wire(line));
		}
	}
	wires
}

fn create_graphviz(filepath: &str, wires: &Vec<Wire>) {
	let file = fs::File::create(filepath);
	match writeln!(file.as_ref().expect("REASON"), "{}", "graph AOC25 {") {
		Ok(()) => {},
		Err(..) => return,
	}
	for wire in wires {
		for end in &wire.ends {
			match writeln!(file.as_ref().expect("REASON"), "{}", format!("	{} -- {};", wire.start, end)) {
			Ok(()) => {},
			Err(..) => return,
			}
		}
	}
	match writeln!(file.as_ref().expect("REASON"), "{}", "}") {
		Ok(()) => {},
		Err(..) => return,
	}
}

fn is_forb(w1: &str, w2: &str, forb: &[(&str, &str); 3]) -> bool {
	for f in forb {
		if (f.0 == w1 && f.1 == w2) || (f.1 == w1 && f.0 == w2) {
			return true;
		}
	}
	false 
}

fn partitions(wires: &Vec<Wire>, forb: &[(&str, &str); 3]) -> u64 {
	let mut found: HashSet<String> = HashSet::new();
	let mut queue: VecDeque<String> = VecDeque::new();
	found.insert(wires[0].start.clone());
	queue.push_back(wires[0].start.clone());
	while !queue.is_empty() {
		let wire = queue.pop_front().unwrap();
		for w in wires {
			if wire == w.start {
				for e in &w.ends {
					if !is_forb(&wire, &e.clone(), forb) && !found.contains(&e.clone()) {
						found.insert(e.clone());
						queue.push_back(e.clone());
					}
				}
			}
			else if w.ends.clone().into_iter().filter(|e| &wire == e).count() > 0 {
				for e in &w.ends {
					if &wire == e && !is_forb(&w.start, &e.clone(), forb) && !found.contains(&w.start) {
						found.insert(w.start.clone());
						queue.push_back(w.start.clone());
					}
				}
			}
		}
	}
	let mut all: HashSet<String> = HashSet::new();
	for w in wires {
		all.insert(w.start.clone());
		for e in &w.ends {
			all.insert(e.clone());
		}
	}
	(found.len() * (all.len() - found.len())) as u64
}

fn part1() {
	let wires = read_file("INPUT");
	create_graphviz("graph.dot", &wires);
	// This is only my solution. Run `dot -Kneato -Tsvg graph.dot -o graph.svg` to create your graph.
	let forb: [(&str, &str); 3] = [("dlv", "tqh"), ("ngp", "bmd"), ("tqr", "grd")];
	println!("Run `dot -Kneato -Tsvg graph.dot -o graph.svg` to see three edges.");
	println!("Part 1: {}", partitions(&wires, &forb));
}

fn part2() {
	println!("Part 2: {}", "PUSH THE RED BUTTON.");
}

fn main() {
	println!("Year 2023 day 25 - Snowverload");
	part1();
	part2();
}
