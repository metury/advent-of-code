use std::fs;
use std::collections::HashSet;

const EMPTY: char = '.';
const NON_EMPTY: char = '#';

type Grid<T> = Vec<Vec<T>>;
type Position = (usize, usize);

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

fn find_start_end(map: &Grid<char>) -> (Position, Position) {
	let mut start: Position = (0,0);
	let mut end: Position = (0,0);
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

fn add(a: Position, b: (i8, i8)) -> Position {
	((a.0 as i64 + b.0 as i64) as usize, (a.1 as i64 + b.1 as i64) as usize)
}

fn step(map: &Grid<char>, position: Position, ignore_slopes: bool) -> Vec<Position> {
	let mut positions: Vec<Position> = vec!();
	const LEN: usize = 4;
	let neighbors: [(i8, i8); LEN] = [(0,-1), (0,1), (1,0), (-1,0)];
	let slopes: [char; LEN] = ['<', '>', 'v', '^'];
	for i in 0 .. LEN {
		let pos: Position = add(position, neighbors[i]);
		if pos.0 < map.len() && pos.1 < map[pos.0].len() {
			if ignore_slopes && map[pos.0][pos.1] != NON_EMPTY {
				positions.push(pos);
			}
			else if !ignore_slopes && (map[pos.0][pos.1] == EMPTY || map[pos.0][pos.1] == slopes[i]) {
				positions.push(pos);
			}
		}
	}
	positions
}

fn longest_path(map: &Grid<char>, visited: &mut HashSet<Position>, pos: Position, max: &mut u64, len: u64, end: &Position, ignore_slopes: bool) {
	if pos == *end {
		if *max < len {
			*max = len;
		}
		return;
	}
	let neighbors = step(map, pos, ignore_slopes);
	for n in neighbors {
		if !visited.contains(&n) {
			visited.insert(n);
			longest_path(map, visited, n, max, len+1, end, ignore_slopes);
			visited.remove(&n);
		}
	}
}

fn part1() {
	let map = read_file("INPUT");
	let (start, end) = find_start_end(&map);
	let mut set: HashSet<Position> = HashSet::new();
	let mut max: u64 = 0;
	longest_path(&map, &mut set, start, &mut max, 0, &end, false);
	println!("Part 1: {}", max);
}

fn part2() {
	let map = read_file("INPUT");
	let (start, end) = find_start_end(&map);
	let mut set: HashSet<Position> = HashSet::new();
	let mut max: u64 = 0;
	longest_path(&map, &mut set, start, &mut max, 0, &end, true);
	println!("Part 2: {}", max);
}

fn main() {
	println!("Year 2023 day 23 - A Long Walk");
	part1();
	part2();
}
