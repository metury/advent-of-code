use std::fs;
use std::collections::{HashMap, VecDeque, HashSet};
use std::io::Write;

const BROADCASTER: &str = "broadcaster";
const RX : &str = "rx";

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
	hash_map.insert(RX.to_string(), Node { name: RX.to_string(), node_type: NodeType::Broadcast, on: false, next_nodes: vec!(), predecessors: HashMap::new()});
	return hash_map;
}

fn find_predecessors(hash_map: &mut HashMap<String, Node>) {
	let mut name = BROADCASTER.to_string();
	let mut set: HashSet<String> = HashSet::new();
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
			if let Some(new_node) = hash_map.get_mut(n) {
				new_node.predecessors.insert(name.clone(), Pulse::Low);
				queue.push_back(n.clone());
			}
		}
	}
}

fn create_graphviz(hash_map: &HashMap<String, Node>, filepath: &str) {
	let file = fs::File::create(filepath);
	match writeln!(file.as_ref().expect("REASON"), "{}", "digraph AOC {") {
		Ok(()) => {},
		Err(..) => return,
	}
	for n in hash_map {
		let mut color: &str;
		match n.1.node_type {
			NodeType::Broadcast => color = "red",
			NodeType::Conjuction => color = "blue",
			NodeType::FlipFlop => color = "green",
		}
		if n.1.name == RX {
			color = "orange";
		}
		match writeln!(file.as_ref().expect("REASON"), "{}", format!("	{} [color={}]", n.1.name, color)) {
			Ok(()) => {},
			Err(..) => return,
		}
	}
	let mut name = BROADCASTER.to_string();
	let mut set: HashSet<String> = HashSet::new();
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
			match writeln!(file.as_ref().expect("REASON"), "{}", format!("	{} -> {};", name, n)) {
				Ok(()) => {},
				Err(..) => return,
			}
			queue.push_back(n.clone());
		}
	}
	match writeln!(file.as_ref().expect("REASON"), "{}", "}") {
		Ok(()) => {},
		Err(..) => return,
	}
}

fn all_highs(hash_map: &HashMap<String, Pulse>) -> bool {
	hash_map.into_iter().map(|x| x.1 == &Pulse::High).fold(true, |acc, x| acc && x)
}

fn bfs(hash_map: &mut HashMap<String, Node>) -> (i64, i64) {
	let mut lows: i64 = 1;
	let mut highs: i64 = 0;
	let mut name = BROADCASTER.to_string();
	let mut pulse: Pulse = Pulse::Low;
	let mut queue: VecDeque<(String, Pulse)> = VecDeque::new();
	queue.push_back((name.clone(), pulse));
	while !queue.is_empty() {
		(name, pulse) = queue.pop_front().unwrap();
		let next_nodes = &hash_map[&name].next_nodes.clone();
		match &hash_map[&name].node_type {
			NodeType::FlipFlop => {
				if pulse == Pulse::Low {
					hash_map.get_mut(&name).unwrap().on = !&hash_map[&name].on;
					if hash_map[&name].on {
						pulse = Pulse::High;
					}
					else {
						pulse = Pulse::Low;
					}
				}
				else {
					continue;
				}
			}
			NodeType::Conjuction => {
				if all_highs(&hash_map[&name].predecessors) {
					pulse = Pulse::Low;
				}
				else {
					pulse = Pulse::High;
				}
			}
			NodeType::Broadcast => pulse = pulse,
		}
		if pulse == Pulse::High {
			highs += next_nodes.len() as i64;
		}
		else {
			lows += next_nodes.len() as i64;
		}
		for n in next_nodes {
			if let Some(new_node) = hash_map.get_mut(n) {
				new_node.predecessors.insert(name.clone(), pulse.clone());
				queue.push_back((n.clone(), pulse.clone()));
			}
		}
	}
	(lows, highs)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
	while b != 0 {
		let temp = b;
		b = a % b;
		a = temp;
	}
	a
}

fn lcm(a: u64, b: u64) -> u64 {
	(a * b) / gcd(a,b)
}

fn lcm_vec(vec: &Vec<u64>) -> u64 {
	vec.into_iter().fold(1, |acc, x| lcm(*x, acc))
}

fn part1() {
	let mut hash_map = read_file("INPUT");
	find_predecessors(&mut hash_map);
	let mut total_lows: i64 = 0;
	let mut total_highs: i64 = 0;
	for _ in 0 .. 1000 {
		let (low, high) = bfs(&mut hash_map);
		total_lows += low;
		total_highs += high;
	}
	println!("Part 1: {}", total_lows * total_highs);
}

fn part2() {
	let hash_map = read_file("INPUT");
	create_graphviz(&hash_map, "graph.dot");
	let vec: Vec<u64> = [2, 3, 4, 6, 8, 9, 10, 11, 12, 13, 14].to_vec();
	println!("Part 2: {}", lcm_vec(&vec));
}

fn main() {
	println!("Year 2023 day 20 - Pulse Propagation");
	part1();
	part2();
}
