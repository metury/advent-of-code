use std::fs;
use std::collections::HashSet;

const EMPTY: char = '.';
const NON_EMPTY: char = '#';

type Grid<T> = Vec<Vec<T>>;
type Vertex = (usize, usize);

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Edge {
	from: Vertex,
	to: Vertex,
	len: u64,
	slope: bool,
}

#[derive(Debug)]
struct Graph {
	edges: HashSet<Edge>,
}

impl Graph {
	fn add_edge(&mut self, from: Vertex, to: Vertex, len: u64, slope: bool) {
		self.edges.insert(Edge{from: from, to: to, len: len, slope: slope});
	}

	fn add_bi_edge(&mut self, from: Vertex, to: Vertex, len: u64, slope: bool) {
		self.edges.insert(Edge{from: from, to: to, len: len, slope: slope});
		self.edges.insert(Edge{from: to, to: from, len: len, slope: slope});
	}
}

/**
 * Read the input grid.
 */
fn read_file(filepath: &str) -> Grid<char> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	let mut map: Grid<char> = vec![];
	for line in lines{
		let map_line: Vec<char> = line.chars().collect();
		if map_line.len() > 0 {
			map.push(map_line);
		}
	}
	map
}

/**
 * Find the start and end postion in the grid.
 */
fn find_start_end(map: &Grid<char>) -> (Vertex, Vertex) {
	let mut start: Vertex = (0,0);
	let mut end: Vertex = (0,0);
	for i in 0 .. map[0].len() {
		if map[0][i] == EMPTY {
			start = (0, i);
			break;
		}
	}
	for i in 0 .. map[map.len() - 1].len() {
		if map[map.len() - 1][i] == EMPTY {
			end = (map.len() - 1, i);
			break;
		}
	}
	(start, end)
}

/**
 * Add vertex to a shift.
 */
fn add(a: Vertex, b: (i8, i8)) -> Vertex {
	((a.0 as i64 + b.0 as i64) as usize, (a.1 as i64 + b.1 as i64) as usize)
}

/**
 * Find the neigbours of a given vertex.
 * Return all the neigbours and if they are blocked by a slope.
 */
fn step(map: &Grid<char>, vertex: Vertex, slope: bool) -> Vec<(Vertex, bool)> {
	let mut vertices: Vec<(Vertex, bool)> = vec!();
	const LEN: usize = 4;
	let neighbors: [(i8, i8); LEN] = [(0,-1), (0,1), (1,0), (-1,0)];
	let slopes: [char; LEN] = ['<', '>', 'v', '^'];
	for i in 0 .. LEN {
		let v: Vertex = add(vertex, neighbors[i]);
		if v.0 < map.len() && v.1 < map[v.0].len() {
			if !slope && map[v.0][v.1] == NON_EMPTY {
				vertices.push((v, slope));
			}
			else if slope && (map[v.0][v.1] == EMPTY || map[v.0][v.1] == slopes[i]) {
				vertices.push((v, map[v.0][v.1] == slopes[i]));
			}
		}
	}
	vertices
}

/**
 * Create a graph.
 */
fn create_graph(map: &Grid<char>, slope: bool, graph: &mut Graph, visited: &mut HashSet<Vertex>, edge: Edge) {
	let neighbors: Vec<(Vertex, bool)> = step(map, edge.to, slope).into_iter().filter(|(v,_)| !visited.contains(v)).collect();
	if neighbors.len() > 1 {
		// There are at least two neighbors so they branch out.
		for n in neighbors {
			// For each one crate an edge (from - n) and possibly (n - from).
			if edge.slope || n.1 {
				graph.add_edge(edge.from, n.0, edge.len + 1, true);
			}
			else {
				graph.add_bi_edge(edge.from, n.0, edge.len + 1, false);
			}
			visited.insert(n.0);
			create_graph(map, slope, graph, visited, Edge{from: n.0, to: n.0, len: 0, slope: false});
			visited.remove(&n.0);
		}
	}
	else if neighbors.len() == 1 {
		// Contract the edge.
		visited.insert(neighbors[0].0);
		create_graph(map, slope, graph, visited, Edge{from: edge.from, to: neighbors[0].0, len: edge.len + 1, slope: false});
	}
	else{
		// It is the end.
		visited.insert(edge.to);
		if edge.slope {
			graph.add_edge(edge.from, edge.to, edge.len + 1, true);
		}
		else {
			graph.add_bi_edge(edge.from, edge.to, edge.len + 1, false);
		}
	}
}

/**
 * Compute the longest path for the given graph.
 */
fn longest_path(graph: &Graph, visited: &mut HashSet<Vertex>, v: Vertex, end: &Vertex, max: &mut u64, len: u64) {
	if *end == v {
		if *max < len {
			*max = len;
		}
		return;
	}
	let neighbors: Vec<Edge> = graph.edges.clone().into_iter().filter(|e| e.from == v && !visited.contains(&e.to)).collect();
	for e in neighbors {
		visited.insert(e.to);
		longest_path(graph, visited, e.to, end, max, len + e.len);
		visited.remove(&e.to);
	}
}

fn part1() {
	let map = read_file("INPUT");
	let (start, end) = find_start_end(&map);
	let mut graph = Graph{edges: HashSet::new()};
	let mut visited: HashSet<Vertex> = HashSet::new();
	let edge = Edge{from: start, to: start, len: 0, slope: false};
	create_graph(&map, true, &mut graph, &mut visited, edge);
	let mut max: u64 = 0;
	visited.clear();
	longest_path(&graph, &mut visited, start, &end, &mut max, 0);
	//println!("{:?}", graph);
	println!("Part 1: {}", max);
}

fn part2() {
	let map = read_file("INPUT");
	let (start, end) = find_start_end(&map);
	let mut graph = Graph{edges: HashSet::new()};
	let mut visited: HashSet<Vertex> = HashSet::new();
	let edge = Edge{from: start, to: start, len: 0, slope: false};
	create_graph(&map, false, &mut graph, &mut visited, edge);
	//println!("{:?}", graph);
	let mut max: u64 = 0;
	visited.clear();
	longest_path(&graph, &mut visited, start, &end, &mut max, 0);
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 23 - A Long Walk");
	part1();
	part2();
}
