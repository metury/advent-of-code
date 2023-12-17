use std::fs;
use petgraph::graph::{Graph, NodeIndex};
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

struct Node{
	position: (usize, usize),
	movement: (i64, i64),
	limit: i8,
}

fn create_graph(grid: &Vec<Vec<i8>>) -> (Graph<Node, i64>, NodeIndex) {
	let mut graph = Graph::<Node,i64>::new();
	let moves = [(0,1), (0,-1), (1,0), (-1,0)];
	let mut nodes: Vec<(Node, NodeIndex)> = vec!();
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
	for i in 0..nodes.len() - 1 {
		for j in 0..nodes.len() - 1 {
			let pos_i = nodes[i].0.position;
			let pos_j = nodes[j].0.position;
			let mov_j = nodes[j].0.movement;
			let limit_i = nodes[j].0.limit;
			let limit_j = nodes[j].0.limit;
			let same_row: bool = pos_i.0 as i64 + mov_j.0 == pos_j.0 as i64
			                  && pos_i.1 as i64 + mov_j.1 == pos_j.1 as i64
			                  && limit_i < 2 && limit_i + 1 == limit_j;
			let left: bool = pos_i.0 as i64 + mov_j.1 == pos_j.0 as i64
			              && pos_j.0 as i64 + mov_j.0 == pos_j.0 as i64
			              && limit_j == 0;
			let right: bool = pos_i.0 as i64 - mov_j.1 == pos_j.0 as i64
			               && pos_j.0 as i64 - mov_j.0 == pos_j.0 as i64
			               && limit_j == 0;
			if same_row || left || right {
				graph.add_edge(nodes[i].1, nodes[j].1, grid[pos_j.0][pos_j.1] as i64);
			}
		}
	}
	return (graph, nodes[0].1);
}

fn part1(){
	let (grid, mut paths) = read_file("INPUT");
	let (graph, start_node) = create_graph(&grid);
	let distances = dijkstra(&graph, start_node, None, |_| 1);
	println!("{:?}", distances);
	println!("Part 1: {}", 0);
}

fn part2(){
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 17 - Clumsy Crucible");
	part1();
	part2();
}
