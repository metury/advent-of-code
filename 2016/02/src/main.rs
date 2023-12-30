use std::fs;

const EMPTY: char = '$';

const DIALPAD: [[char; 7]; 7] = [[EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
                                 [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
                                 [EMPTY, EMPTY,   '1',   '2',   '3', EMPTY, EMPTY],
                                 [EMPTY, EMPTY,   '4',   '5',   '6', EMPTY, EMPTY],
                                 [EMPTY, EMPTY,   '7',   '8',   '9', EMPTY, EMPTY],
                                 [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
                                 [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY]];

const DIALPAD_ADVANCED: [[char; 7]; 7] = [[EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
                                          [EMPTY, EMPTY, EMPTY,   '1', EMPTY, EMPTY, EMPTY],
                                          [EMPTY, EMPTY,   '2',   '3',   '4', EMPTY, EMPTY],
                                          [EMPTY,   '5',   '6',   '7',   '8',   '9', EMPTY],
                                          [EMPTY, EMPTY,   'A',   'B',   'C', EMPTY, EMPTY],
                                          [EMPTY, EMPTY, EMPTY,   'D', EMPTY, EMPTY, EMPTY],
                                          [EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY, EMPTY]];

const MIDDLE: (usize, usize) = (3,3);
const MIDDLE_ADVANCED: (usize, usize) = (3,1);

fn read_file(filepath: &str) -> Vec<String> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n')
		.filter(|c| c.len() > 0)
		.map(|c| c.to_string()).collect();
	lines
}

fn step(start: (usize, usize), pad: &[[char; 7]; 7], c: char) -> (usize, usize) {
	let mut pos = start;
	match c {
		'U' => pos.0 = pos.0 - 1,
		'D' => pos.0 = pos.0 + 1,
		'L' => pos.1 = pos.1 - 1,
		'R' => pos.1 = pos.1 + 1,
		 _  => pos = pos,
	 }
	 if pad[pos.0][pos.1] != EMPTY {
		 return pos;
	 }
	 start
}

fn move_in_dialpad(start: (usize, usize), pad: &[[char; 7]; 7], line: &str) -> (usize, usize) {
	let mut pos = start;
	for c in line.chars() {
		pos = step(pos, pad, c);
	}
	pos
}

fn part1() {
	let lines = read_file("INPUT");
	let mut code: String = "".to_string();
	let mut pos = MIDDLE;
	for line in lines {
		pos = move_in_dialpad(pos, &DIALPAD, &line);
		code.push(DIALPAD[pos.0][pos.1]);
	}
	println!("Part 1: {}", code);
}

fn part2() {
	let lines = read_file("INPUT");
	let mut code: String = "".to_string();
	let mut pos = MIDDLE_ADVANCED;
	for line in lines {
		pos = move_in_dialpad(pos, &DIALPAD_ADVANCED, &line);
		code.push(DIALPAD_ADVANCED[pos.0][pos.1]);
	}
	println!("Part 2: {}", code);
}

fn main() {
	println!("Year 2016 day 2 - Bathroom Security");
	part1();
	part2();
}
