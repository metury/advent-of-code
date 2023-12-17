use std::fs;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::algo::dijkstra;

fn read_file(filepath: &str) -> (Vec<Vec<i8>>, Vec<Vec<i64>>){
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	let mut grid: Vec<Vec<i8>> = vec![];
	let mut paths: Vec<Vec<i64>> = vec![];
	for line in lines{
		let mut grid_line: Vec<i8> = vec!();
		let mut paths_line: Vec<i64> = vec!();
		for c in line.chars(){
			grid_line.push((c as i64 - '0' as i64) as i8);
			paths_line.push(i64::MAX);
		}
		if grid_line.len() > 0{
			grid.push(grid_line);
			paths.push(paths_line);
		}
	}
	return (grid, paths);

}

#[derive(Debug)]
struct Node{
	position: (usize, usize),
	movement: (i64, i64),
	limit: i8,
}

fn create_graph(grid: &Vec<Vec<i8>>) -> (DiGraph<Node, i64>, NodeIndex, NodeIndex) {
	let mut graph = DiGraph::<Node,i64>::new();
	let moves = [(0,1), (0,-1), (1,0), (-1,0)];
	let mut nodes: Vec<(Node, NodeIndex)> = vec!();
	let starting_node = graph.add_node(Node{position: (0,0), movement: (0,0), limit: 4});
	let ending_node = graph.add_node(Node{position: (0,0), movement: (0,0), limit: 5});
	for i in 0..grid.len() {
		for j in 0..grid[i].len() {
			for m in moves {
				for k in 0..3 {
					let a = graph.add_node(Node{position: (i,j), movement: m, limit: k});
					nodes.push((Node{position: (i,j), movement: m, limit: k}, a));
				}
			}
		}
	}
	for i in 0..nodes.len() {
		let pos_i = nodes[i].0.position;
		let mov_i = nodes[i].0.movement;
		let limit_i = nodes[i].0.limit;
		for j in 0..nodes.len() {
			let pos_j = nodes[j].0.position;
			let mov_j = nodes[j].0.movement;
			let limit_j = nodes[j].0.limit;
			let same_row: bool = pos_i.0 as i64 + mov_j.0 == pos_j.0 as i64
			                  && pos_i.1 as i64 + mov_j.1 == pos_j.1 as i64
			                  && mov_i == mov_j
			                  && limit_j < 2 && limit_i + 1 == limit_j;
			let left: bool = pos_i.0 as i64 + mov_j.0 == pos_j.0 as i64
			              && pos_i.1 as i64 + mov_j.1 == pos_j.1 as i64
			              && mov_i.0 == mov_j.1 && mov_i.1 == mov_j.0
			              && limit_j == 0;
			let right: bool = pos_i.0 as i64 + mov_j.0 == pos_j.0 as i64
			               && pos_i.1 as i64 + mov_j.1 == pos_j.1 as i64
			               && mov_i.0 == -mov_j.1 && mov_i.1 == -mov_j.0
			               && limit_j == 0;
			if same_row || left || right {
				graph.add_edge(nodes[i].1, nodes[j].1, grid[pos_j.0][pos_j.1] as i64);
			}
		}
		if pos_i == (0,0) && (mov_i == (0,1) || mov_i == (1,0)) && limit_i == 0 {
			graph.add_edge(starting_node, nodes[i].1, 0);
		}
		if pos_i == (grid.len() - 1, grid.len() - 1) {
			graph.add_edge(nodes[i].1, ending_node, 0);
		}
	}
	return (graph, starting_node, ending_node);
}

fn part1(){
	let (grid, mut paths) = read_file("INPUT");
	let (graph, start_node, ending_node) = create_graph(&grid);
	let distances = dijkstra(&graph, start_node, None, |e| *e.weight() );
	println!("Part 1: {}", distances[&ending_node]);
}

fn part2(){
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 17 - Clumsy Crucible");
	part1();
	part2();
}
