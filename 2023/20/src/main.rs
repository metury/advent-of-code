use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};

#[derive(Debug, Eq, PartialEq, Clone)]
enum NodeType {
	FlipFlop,
	Conjuction,
	Broadcast,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Pulse {
	Low,
	High,
}

#[derive(Debug, Clone)]
struct Node {
	name: String,
	node_type: NodeType,
	on: bool,
	next_nodes: Vec<String>,
	predecessors: HashMap<String, Pulse>,
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
	let mut set: HashSet<String> = HashSet::new();
	let mut node: &Node;
	let mut queue: VecDeque<String> = VecDeque::new();
	queue.push_back(name.clone());
	while !queue.is_empty() {
		name = queue.pop_front().unwrap().to_string();
		if set.contains(&name) {
			continue;
		}
		set.insert(name.clone());
		let next_nodes = &hash_map[&name].next_nodes.clone();
		for n in next_nodes {
			let mut new_node = hash_map.get_mut(n).unwrap();
			new_node.predecessors.insert(name.clone(), Pulse::Low);
			queue.push_back(n.clone());
		}
	}
}

fn all_highs(hash_map: &HashMap<String, Pulse>) -> bool {
	hash_map.into_iter().map(|x| x.1 == &Pulse::High).fold(true, |acc, x| acc && x)
}

fn bfs(hash_map: &HashMap<String, Node>) {
	let mut name = "broadcaster".to_string();
	let mut pulse: Pulse = Pulse::Low;
	let mut node: &Node;
	let mut queue: VecDeque<(String, Pulse)> = VecDeque::new();
	queue.push_back((name.clone(), pulse));
	while !queue.is_empty() {
		(name, pulse) = queue.pop_front().unwrap().to_string();
		let next_nodes = &hash_map[&name].next_nodes.clone();
		match node.node_type {
			NodeType::FlipFlop => {
				if pulse == Pulse::Low {
					node.on = !node.on;
					if node.on {
						pulse = Pulse::High;
					}
					else {
						pulse = Pulse::Low;
					}
				}
			}
			NodeType::Conjuction => {
			NodeType::Broadcast => todo!(),
		}
		for n in next_nodes {
			let mut new_node = hash_map.get_mut(n).unwrap();
			// Update for conju
			queue.push_back(n.clone());
		}
	}
}

fn part1() {
	let mut hash_map = read_file("INPUT");
	find_predecessors(&mut hash_map);
	for node in hash_map {
		println!("{:?}", node);
	}
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
