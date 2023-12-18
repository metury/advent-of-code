use std::fs;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

type Grid<T> = Vec<Vec<T>>;
type Position = (usize, usize);

fn read_file(filepath: &str) -> Grid<usize>{
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	let mut grid: Grid<usize> = vec![];
	for line in lines{
		let mut grid_line: Vec<usize> = vec!();
		for c in line.chars(){
			grid_line.push(c.to_digit(10).unwrap() as usize);
		}
		if grid_line.len() > 0{
			grid.push(grid_line);
		}
	}
	return grid;
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
enum Direction{
	Up,
	Down,
	Right,
	Left,
}

#[derive(Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
struct State {
	position: Position,
	remaining_up: usize,
	remaining_right: usize,
	remaining_down: usize,
	remaining_left: usize,
	direction: Direction,
	moves_in_current_direction: usize,
}

fn get_neighbouring_states(state: State, map: &Grid<usize>, min: usize, max: usize) -> Vec<State> {
	let mut neighbours: Vec<State> = vec!();

	if state.moves_in_current_direction < min {
	match state.direction {
			Direction::Up => {
				if state.position.0 >= 1 {
					neighbours.push(State {
							position: (state.position.0 - 1, state.position.1),
							remaining_up: state.remaining_up - 1,
							remaining_right: max,
							remaining_down: max,
							remaining_left: max,
							direction: Direction::Up,
							moves_in_current_direction: state.moves_in_current_direction + 1,
					});
				}
				return neighbours;
				}
			Direction::Right => {
				if state.position.1 < map[0].len() - 1 {
					neighbours.push(State {
							position: (state.position.0, state.position.1 + 1),
							remaining_up: max,
							remaining_right: state.remaining_right - 1,
							remaining_down: max,
							remaining_left: max,
							direction: Direction::Right,
							moves_in_current_direction: state.moves_in_current_direction + 1,
					});
				}
				return neighbours;
			}
			Direction::Down => {
				if state.position.0 < map.len() - 1 {
					neighbours.push(State {
							position: (state.position.0 + 1, state.position.1),
							remaining_up: max,
							remaining_right: max,
							remaining_down: state.remaining_down - 1,
							remaining_left: max,
							direction: Direction::Down,
							moves_in_current_direction: state.moves_in_current_direction + 1,
					});
				}
				return neighbours;
			}
			Direction::Left => {
				if state.position.1 >= 1 {
					neighbours.push(State {
							position: (state.position.0, state.position.1 - 1),
							remaining_up: max,
							remaining_right: max,
							remaining_down: max,
							remaining_left: state.remaining_left - 1,
							direction: Direction::Left,
							moves_in_current_direction: state.moves_in_current_direction + 1,
					});
				}
				return neighbours;
			}
		}
	}

	if state.remaining_up >= 1 && state.position.0 >= 1 && state.direction != Direction::Down {
		neighbours.push(State {
				position: (state.position.0 - 1, state.position.1),
				remaining_up: state.remaining_up - 1,
				remaining_right: max,
				remaining_down: max,
				remaining_left: max,
				direction: Direction::Up,
				moves_in_current_direction: if state.direction == Direction::Up {
					 state.moves_in_current_direction + 1
				} else {
					 1
				},
		});
	}

	if state.remaining_right >= 1
	&& state.position.1 < map[0].len() - 1
	&& state.direction != Direction::Left {
		neighbours.push(State {
			position: (state.position.0, state.position.1 + 1),
			remaining_up: max,
			remaining_right: state.remaining_right - 1,
			remaining_down: max,
			remaining_left: max,
			direction: Direction::Right,
			moves_in_current_direction: if state.direction == Direction::Right {
				 state.moves_in_current_direction + 1
			} else { 1 }, });
	}

	if state.remaining_down >= 1
	&& state.position.0 < map.len() - 1
	&& state.direction != Direction::Up {
		neighbours.push(State {
			position: (state.position.0 + 1, state.position.1),
			remaining_up: max,
			remaining_right: max,
			remaining_down: state.remaining_down - 1,
			remaining_left: max,
			direction: Direction::Down,
			moves_in_current_direction: if state.direction == Direction::Down {
				 state.moves_in_current_direction + 1
			} else { 1 }, });
	}

	if state.remaining_left >= 1 && state.position.1 >= 1 && state.direction != Direction::Right {
		neighbours.push(State {
			position: (state.position.0, state.position.1 - 1),
			remaining_up: max,
			remaining_right: max,
			remaining_down: max,
			remaining_left: state.remaining_left - 1,
			direction: Direction::Left,
			moves_in_current_direction: if state.direction == Direction::Left {
				 state.moves_in_current_direction + 1
			} else { 1 }, });
	}

	neighbours
}

fn get_cost(map: &Grid<usize>, source: Position, destination: Position, min: usize, max: usize) -> usize {
	let mut costs: HashMap<State, usize> = HashMap::new();
	let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

	let start_state_right = State {
		position: source,
		remaining_up: max,
		remaining_right: max,
		remaining_down: max,
		remaining_left: max,
		direction: Direction::Right,
		moves_in_current_direction: 0,
	};
	let start_state_down = State {
		position: source,
		remaining_up: max,
		remaining_right: max,
		remaining_down: max,
		remaining_left: max,
		direction: Direction::Down,
		moves_in_current_direction: 0,
	};

	costs.insert(start_state_right, 0);
	costs.insert(start_state_down, 0);
	heap.push(Reverse((0, start_state_right)));
	heap.push(Reverse((0, start_state_down)));

	while let Some(Reverse((curr_cost, curr_state))) = heap.pop() {
		if curr_state.position == destination && curr_state.moves_in_current_direction >= min {
			return curr_cost;
		}

		if curr_cost > *costs.get(&curr_state).unwrap_or(&usize::MAX) {
			continue;
		}

		let neighbours = get_neighbouring_states(curr_state, map, min, max);

		for neighbour in neighbours {
			let next_pos = neighbour.position;
			let next_cost = curr_cost + map[next_pos.0][next_pos.1];
			if next_cost < *costs.get(&neighbour).unwrap_or(&usize::MAX) {
				heap.push(Reverse((next_cost, neighbour)));
				costs.insert(neighbour, next_cost);
			}
		}
	}
	usize::MAX
}

fn part1(){
	let grid = read_file("INPUT");
	println!("Part 1: {}", get_cost(&grid, (0,0), (grid.len() - 1, grid.len() - 1), 0, 3));
}

fn part2(){
	let grid = read_file("INPUT");
	println!("Part 2: {}", get_cost(&grid, (0,0), (grid.len() - 1, grid.len() - 1), 4, 10));
}

fn main() {
	println!("Year 2023 day 17 - Clumsy Crucible");
	part1();
	part2();
}
