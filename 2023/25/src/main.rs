use std::fs;
use std::collections::{HashMap, HashSet, VecDeque};
use rand::random;
use std::io::Write;

const PAIRS: [(&str, &str); 3] = [("dlv", "tqh"), ("ngp", "bmd"), ("tqr", "grd")];

#[derive(Debug, Clone)]
struct Vertex {
	id: u64,
	counter: u64,
}

#[derive(Debug, Clone)]
struct MultiEdge {
	id_from: u64,
	id_to: u64,
	counter: u64,
}

#[derive(Debug, Clone)]
struct Graph {
	vertices: HashMap<u64, Vertex>,
	edges: Vec<MultiEdge>,
	vertex_size: u64,
	edge_size: u64,
}

fn parse_wire(line: &str, indexing: &mut u64, indexes: &mut HashMap<String, u64>, graph: &mut Graph) {
	let from_to: Vec<&str> = line.split(": ").collect();
	let start = from_to[0];
	if !indexes.contains_key(start) {
		indexes.insert(start.to_string(), *indexing);
		graph.vertices.insert(*indexing, Vertex {id: *indexing, counter: 1});
		graph.vertex_size += 1;
		*indexing += 1;
	}
	let tos: Vec<&str> = from_to[1].split(' ').collect();
	for t in tos {
		if !indexes.contains_key(t) {
			indexes.insert(t.to_string(), *indexing);
			graph.vertices.insert(*indexing, Vertex {id: *indexing, counter: 1});
			graph.vertex_size += 1;
			*indexing += 1;
		}
		let mut add = false;
		for p in PAIRS {
			add |= (start == p.0 && t == p.1) || (start == p.1 && t == p.0);
		}
		if !add {
			graph.edges.push(MultiEdge {id_from: indexes[start], id_to: indexes[t], counter: 1});
			graph.edge_size += 1;
		}
	}
}

fn read_file(filepath: &str) -> Graph {
	let contents = fs::read_to_string(filepath);
	let mut graph = Graph{vertices: HashMap::new(), edges: vec!(), vertex_size: 0, edge_size: 0};
	let mut indexes: HashMap<String, u64> = HashMap::new();
	let mut indexing: u64 = 0;
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		if line != "" {
			parse_wire(line, &mut indexing, &mut indexes, &mut graph);
		}
	}
	graph
}

fn fix_edges(graph: &mut Graph, v: u64) {
	for i in 0 .. graph.edges.len() {
		if i >= graph.edges.len() {
			return;
		}
		if graph.edges[i].id_from == v {
			for j in i+1 .. graph.edges.len() {
				if j >= graph.edges.len() {
					break;
				}
				if graph.edges[j].id_from == v || graph.edges[j].id_to == graph.edges[i].id_to {
					graph.edges[i].counter += graph.edges[j].counter;
					graph.edges.remove(j);
				}
				else if graph.edges[j].id_to == v || graph.edges[j].id_from == graph.edges[i].id_to {
					graph.edges[i].counter += graph.edges[j].counter;
					graph.edges.remove(j);
				}
			}
		}
		if graph.edges[i].id_to == v {
			for j in i+1 .. graph.edges.len() {
				if j >= graph.edges.len() {
					break;
				}
				if graph.edges[j].id_from == v || graph.edges[j].id_to == graph.edges[i].id_from {
					graph.edges[i].counter += graph.edges[j].counter;
					graph.edges.remove(j);
				}
				else if graph.edges[j].id_to == v || graph.edges[j].id_from == graph.edges[i].id_from {
					graph.edges[i].counter += graph.edges[j].counter;
					graph.edges.remove(j);
				}
			}
		}
	}
}

fn contract(graph: &mut Graph, t: u64) {
	while graph.vertex_size > t {
		let index: u64 = random::<u64>() % graph.edges.len() as u64;
		let MultiEdge {id_from: idf, id_to: idt, counter: c} = graph.edges[index as usize];
		graph.edge_size -= c;
		graph.vertex_size -= 1;
		graph.edges.remove(index as usize);
		for e in &mut graph.edges {
			if e.id_from == idt {
				e.id_from = idf;
			}
			else if e.id_to == idt {
				e.id_to = idf;
			}
		}
		graph.vertices.get_mut(&idf).unwrap().counter += graph.vertices[&idt].counter;
		graph.vertices.remove(&idt);
		fix_edges(graph, idf);
	}
}

fn is_connected(graph: &mut Graph) -> bool {
	let mut found: HashSet<u64> = HashSet::new();
	let mut queue: VecDeque<u64> = VecDeque::new();
	let mut vertex_id: u64 = 0;
	for v in &graph.vertices {
		vertex_id = v.1.id;
		break;
	}
	found.insert(vertex_id);
	queue.push_back(vertex_id);
	while !queue.is_empty() {
		let id = queue.pop_front().unwrap();
		for e in &graph.edges {
			if e.id_from == id {
				if !found.contains(&e.id_to) {
					found.insert(e.id_to);
					queue.push_back(e.id_to);
				}
			}
			if e.id_to == id {
				if !found.contains(&e.id_from) {
					found.insert(e.id_from);
					queue.push_back(e.id_from);
				}
			}
		}
	}
	found.len() == graph.vertices.len() as usize
}

fn partitions(graph: &Graph) -> u64 {
	let mut found: HashSet<u64> = HashSet::new();
	let mut queue: VecDeque<u64> = VecDeque::new();
	let mut vertex_id: u64 = 0;
	for v in &graph.vertices {
		vertex_id = v.1.id;
		break;
	}
	found.insert(vertex_id);
	queue.push_back(vertex_id);
	while !queue.is_empty() {
		let id = queue.pop_front().unwrap();
		for e in &graph.edges {
			if e.id_from == id {
				if !found.contains(&e.id_to) {
					found.insert(e.id_to);
					queue.push_back(e.id_to);
				}
			}
			if e.id_to == id {
				if !found.contains(&e.id_from) {
					found.insert(e.id_from);
					queue.push_back(e.id_from);
				}
			}
		}
	}
	let mut sum1: u64 = 0;
	let mut sum2: u64 = 0;
	for v in &graph.vertices {
		if found.contains(&v.1.id) {
			sum1 += v.1.counter;
		} else {
			sum2 += v.1.counter;
		}
	}
	(found.len() * (graph.vertices.len() - found.len())) as u64
}

fn bruteforce(graph: &mut Graph, h: &mut Graph, min: &mut u64, size: u64) {
	while is_connected(graph) {
		for i in 0 .. graph.edges.len() {
			let mut h1 = graph.clone();
			h1.edges.remove(i);
			h1.edge_size -= graph.edges[i].counter;
			bruteforce(&mut h1, h, min, size + graph.edges[i].counter);
		}
	}
	if *min > size {
		*h = graph.clone();
		*min = size;
	}
}

fn karger_stein(graph: &mut Graph) -> (u64, u64) {
	if graph.vertices.len() < 7 {
		let mut min = graph.edge_size;
		let mut h = graph.clone();
		bruteforce(graph, &mut h, &mut min, 0);
		return (min, partitions(&h));
	}
	else {
		let t: u64 = (1_f64 + (graph.vertex_size as f64 / 2_f64.sqrt())).ceil() as u64;
		let mut h1 = graph.clone();
		let mut h2 = graph.clone();
		contract(&mut h1, t);
		contract(&mut h2, t);
		let (v1, res1) = karger_stein(&mut h1);
		let (v2, res2) = karger_stein(&mut h2);
		if v1 < v2 {
			return (v1, res1);
		}
		else {
			return (v2, res2);
		}
	}
}

fn create_graphviz(input: &str, filepath: &str) {
	let file = fs::File::create(filepath);
	match writeln!(file.as_ref().expect("REASON"), "{}", "graph AOC {") {
		Ok(()) => {},
		Err(..) => return,
	}
	let contents = fs::read_to_string(input);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		if line != "" {
			let from_to: Vec<&str> = line.split(": ").collect();
			let from = from_to[0];
			let parts: Vec<&str> = from_to[1].split(' ').collect();
			for p in parts {
				match writeln!(file.as_ref().expect("REASON"), "{}", format!("	{} -- {};", from, p)) {
				Ok(()) => {},
				Err(..) => return,
				}
			}
		}
	}
	match writeln!(file.as_ref().expect("REASON"), "{}", "}") {
		Ok(()) => {},
		Err(..) => return,
	}
}

fn part1() {
	let mut graph = read_file("INPUT");
	create_graphviz("INPUT", "graph.dot");
	//karger_stein(&mut graph);
	println!("Part 1: {}", partitions(&graph));
}

fn part2() {
	println!("Part 2: {}", "Push the red button.");
}

fn main() {
	println!("Year 2023 day 25 - Snowverload");
	part1();
	part2();
}
