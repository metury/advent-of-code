use std::fs;

fn read_file(filepath: &str) -> String{
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let mut lines = binding.split('\n');
	return lines.next().unwrap().to_string();
}

fn floor_number(string: &str) -> i64 {
	let mut floor: i64 = 0;
	for c in string.chars() {
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_   => floor = floor,
		}
	}
	floor
}

fn basement(string: &str) -> i64 {
	let mut floor: i64 = 0;
	let mut i: i64 = 0;
	for c in string.chars() {
		i += 1;
		match c {
			'(' => floor += 1,
			')' => floor -= 1,
			_   => floor = floor,
		}
		if floor < 0 {
			return i;
		}
	}
	-1
}

fn part1(){
	let line = read_file("INPUT");
	println!("Part 1: {}", floor_number(&line));
}

fn part2(){
	let line = read_file("INPUT");
	println!("Part 2: {}", basement(&line));
}

fn main() {
	println!("Year 2015 day 01 - Not Quite Lisp");
	part1();
	part2();
}
