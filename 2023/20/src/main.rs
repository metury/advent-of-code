use std::fs;
use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq)]
enum NodeType {
	FlipFlop,
	Conjuction,
	Broadcast,
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Node {
	name: String,
	node_type: NodeType,
	on: bool,
	next_nodes: Vec<Node>,
	predecessors: Vec<(Node, bool)>,
}

fn parse_node(line: &str) -> Node {
	
}

fn read_file(filepath: &str) -> HashSet<Node> {
	let mut set: HashSet<Node> = HashSet::new();
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		set.insert(parse_node(line));
	}
	return set;
}

fn part1() {
	println!("Part 1: {}", 0);
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 20 - Pulse Propagation");
	part1();
	part2();
}
