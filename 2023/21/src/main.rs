use std::fs;

fn read_file(filepath: &str) -> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		for c in line.chars(){
		}
	}
	return;
}

fn part1() {
	println!("Part 1: {}", 0);
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 21 - Step Counter");
	part1();
	part2();
}
