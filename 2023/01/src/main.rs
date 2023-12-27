use std::fs;
use regex::Regex;

fn read_file(filepath: &str) -> Vec<String> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines: Vec<String> = binding.split('\n')
			.filter(|l| l.len() > 0)
			.map(|l| l.to_string()).collect();
	lines
}

fn part1() {
	let lines = read_file("INPUT");
	let regex = Regex::new(r"[^1-9]").unwrap();
	let mut total: u32 = 0;
	for line in lines {
		let numbers = regex.replace_all(&line, "");
		total += 10 * numbers.chars().nth(0).unwrap().to_digit(10).unwrap();
		total += numbers.chars().nth(numbers.len() - 1).unwrap().to_digit(10).unwrap();
	}
	println!("Part 1: {}", total);
}

fn part2() {
	let lines = read_file("INPUT");
	const LEN: usize = 9;
	let origin: [&str; LEN] = [r"one", r"two", r"three", r"four", r"five", r"six", r"seven", r"eight", r"nine"];
	let new: [&str; LEN] = ["o1ne", "t2wo", "th3ree", "fo4ur", "fi5ve", "s6ix", "se7ven", "ei8ght", "ni9ne"];
	let regex = Regex::new(r"[^1-9]").unwrap();
	let mut total: u32 = 0;
	for line in lines {
		let mut numbers = line.clone();
		for i in 0 .. LEN {
			let nr = Regex::new(origin[i]).unwrap();
			numbers = nr.replace_all(&numbers, new[i]).to_string();
		}
		numbers = regex.replace_all(&numbers, "").to_string();
		total += 10 * numbers.chars().nth(0).unwrap().to_digit(10).unwrap();
		total += numbers.chars().nth(numbers.len() - 1).unwrap().to_digit(10).unwrap();
	}
	println!("Part 2: {}", total);
}

fn main() {
	println!("Year 2023 day 1 - Trebuchet?!");
	part1();
	part2();
}
