use std::fs;

fn read_file(filepath: &str) -> String {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	binding.split('\n').next().unwrap().to_string()
}

fn part1() {
	let line = read_file("INPUT");
	let mut total: u32 = 0;
	for i in 0 .. line.len() {
		if line.chars().nth(i) == line.chars().nth((i+1) % line.len()) {
			total += line.chars().nth(i).unwrap().to_digit(10).unwrap();
		}
	}
	println!("Part 1: {}", total);
}

fn part2() {
	let line = read_file("INPUT");
	let mut total: u32 = 0;
	for i in 0 .. line.len() {
		if line.chars().nth(i) == line.chars().nth((i + line.len() / 2) % line.len()) {
			total += line.chars().nth(i).unwrap().to_digit(10).unwrap();
		}
	}
	println!("Part 2: {}", total);
}

fn main() {
	println!("Year 2017 day 1 - Inverse Captcha");
	part1();
	part2();
}
