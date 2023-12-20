use std::fs;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Eq, PartialEq, Clone)]
enum NodeType {
	FlipFlop,
	Conjuction,
	Broadcast,
}

#[derive(Debug, Clone)]
struct Node {
	name: String,
	node_type: NodeType,
	on: bool,
	next_nodes: Vec<String>,
	predecessors: HashMap<String, bool>,
}

fn parse_node(line: &str) -> Node {
	let parts: Vec<&str> = line.split(" -> ").collect();
	let node_type: NodeType;
	match parts[0].chars().nth(0).unwrap() {
		'%' => node_type = NodeType::FlipFlop,
		'&' => node_type = NodeType::Conjuction,
		 _  => node_type = NodeType::Broadcast,
	 }
	 let name: String;
	 if node_type != NodeType::Broadcast {
		name = parts[0][1 ..].to_string();
	}
	else {
		name = parts[0].to_string();
	}
	let next_nodes: Vec<String> = parts[1].split(", ").map(|n| n.to_string()).collect();
	Node {name: name, node_type: node_type, on: false, next_nodes: next_nodes, predecessors: HashMap::new()}
}

fn read_file(filepath: &str) -> HashMap<String, Node> {
	let mut hash_map: HashMap<String, Node> = HashMap::new();
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		if line != "" {
			let node = parse_node(line);
			let name = &node.name;
			hash_map.insert(name.to_string(), node);
		}
	}
	return hash_map;
}

fn find_predecessors(hash_map: &mut HashMap<String, Node>) {
	let mut name = "broadcaster".to_string();
	let mut node: &Node;
	let mut queue: VecDeque<String> = VecDeque::new();
	queue.push_back(name.clone());
	while !queue.is_empty() {
		name = queue.pop_front().unwrap().to_string();
		node = &hash_map[&name];
		for n in &node.next_nodes {
			//let new_node = &hash_map[n];
			//new_node.predecessors.insert(name.clone(), false);
			queue.push_back(n.clone()); // Cycles
		}
	}
}

fn part1() {
	let mut hash_map = read_file("INPUT");
	find_predecessors(&mut hash_map);
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
