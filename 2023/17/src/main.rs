use std::fs;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

const LIMIT: i8 = 3;

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
	len: i64,
	position: (usize, usize),
	movement: (i64, i64),
	limit: i8,
}

impl Ord for Node{
	fn cmp(&self, other: &Self) -> Ordering {
		(self.len).cmp(&other.len)
	}
}

impl PartialOrd for Node {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl PartialEq for Node {
	fn eq(&self, other: &Self) -> bool {
		self.len == other.len
	}
}

impl Eq for Node { }

fn create_new_node(grid: &Vec<Vec<i8>>, paths: &mut Vec<Vec<i64>>, node: &Node, heap: &mut BinaryHeap<Node>){
	let len = node.len;
	let pos = node.position;
	let mov = node.movement;
	if pos.0 >= grid.len() || pos.1 >= grid[pos.0].len(){
		return;
	}
	if paths[pos.0][pos.1] < len{
		return;
	}
	let mut new_pos = pos;
	let mut new_mov = mov;
	let mut new_len = len;

	if node.limit < LIMIT {
		new_pos = ((pos.0 as i64 + new_mov.0) as usize, (pos.1 as i64 + new_mov.1) as usize);
		if new_pos.0 < grid.len() && new_pos.1 < grid[pos.0].len() {
			new_len = len + grid[new_pos.0][new_pos.1] as i64;
			heap.push(Node{len: new_len, position: new_pos, movement: new_mov, limit: node.limit + 1});
		}
	}
	new_mov = (mov.1, mov.0);
	new_pos = ((pos.0 as i64 + new_mov.0) as usize, (pos.1 as i64 + new_mov.1) as usize);
	if new_pos.0 < grid.len() && new_pos.1 < grid[pos.0].len() {
		new_len = len + grid[new_pos.0][new_pos.1] as i64;
		heap.push(Node{len: new_len, position: new_pos, movement: new_mov, limit: 1});
	}
	new_mov = (-mov.1, -mov.0);
	new_pos = ((pos.0 as i64 + new_mov.0) as usize, (pos.1 as i64 + new_mov.1) as usize);
	if new_pos.0 < grid.len() && new_pos.1 < grid[pos.0].len() {
		new_len = len + grid[new_pos.0][new_pos.1] as i64;
		heap.push(Node{len: new_len, position: new_pos, movement: new_mov, limit: 1});
	}
}

fn dijkstra(grid: &Vec<Vec<i8>>, paths: &mut Vec<Vec<i64>>){
	let mut heap: BinaryHeap<Node> = BinaryHeap::new();
	paths[0][0] = grid[0][0] as i64;
	let path = paths[0][0];
	let mut node: Node = Node{ len: path + grid[0][1] as i64, position: (0,1), movement: (0,1), limit: 1};
	heap.push(node);
	node = Node{ len: path + grid[1][0] as i64, position: (1,0), movement: (1,0), limit: 1};
	heap.push(node);
	while heap.peek() != None{
		let Some(node) = heap.pop() else { return; };
		create_new_node(grid, paths, &node, &mut heap);
	}
}

fn step(grid: &Vec<Vec<i8>>, paths: &mut Vec<Vec<i64>>, position: (usize, usize), movement: (i8, i8), limit: i8, path: i64){
	let out_of_bounds: bool = position.0 >= grid.len() || position.1 >= grid[position.0].len();
	if out_of_bounds || paths[position.0][position.1] < path + grid[position.0][position.1] as i64{
		return;
	}
	let extended_path = path + grid[position.0][position.1] as i64;
	paths[position.0][position.1] = extended_path;
	let new_pos = ((position.0 as i64 + movement.0 as i64) as usize, (position.1 as i64 + movement.1 as i64) as usize);
	if limit < 3{
		step(grid, paths, new_pos, movement, limit + 1, extended_path);
	}
	let mut new_movement = (movement.1, movement.0);
	step(grid, paths, new_pos, new_movement, 0, extended_path);
	new_movement = (-movement.1, -movement.0);
	step(grid, paths, new_pos, new_movement, 0, extended_path);
}

fn first_step(grid: &Vec<Vec<i8>>, paths: &mut Vec<Vec<i64>>){
	let position: (usize, usize) = (0,0);
	let mut movement: (i8, i8) = (0,1);
	step(grid, paths, position, movement, 0, 0);
	movement = (1,0);
	step(grid, paths, position, movement, 0, 0);
}

fn part1(){
	let (grid, mut paths) = read_file("INPUT");
	//first_step(&grid, &mut paths);
	dijkstra(&grid, &mut paths);
	let result = paths[paths.len() - 1][paths[paths.len() - 1].len() - 1];
	println!("Part 1: {}", result);
}

fn part2(){
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 17 - Clumsy Crucible");
	part1();
	part2();
}
