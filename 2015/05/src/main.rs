use std::fs;
use regex::Regex;
use fancy_regex::Regex as Rgx;

fn read_file(filepath: &str) -> Vec<String> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n')
		.filter(|c| c.len() > 0)
		.map(|c| c.to_string()).collect();
	lines
}

fn nice_string(lines: &Vec<String>) -> u64 {
	let mut total: u64 = 0;
	let vowels = Regex::new(r"a|e|i|o|u").unwrap();
	let pairs = Rgx::new(r"([a-z])\1").unwrap();
	let forb = Regex::new(r"ab|cd|pq|xy").unwrap();
	for line in lines {
		let nr_vowels = vowels.captures_iter(&line).count();
		let exists_pairs = pairs.is_match(&line).unwrap();
		let nr_forb = forb.captures_iter(&line).count();
		if nr_vowels >= 3 && exists_pairs && nr_forb <= 0 {
			total += 1;
		}
	}
	total
}

fn nicer_string(lines: &Vec<String>) -> u64 {
	let mut total: u64 = 0;
	let pairs = Rgx::new(r"([a-z]{2}).*\1").unwrap();
	let middle = Rgx::new(r"([a-z])[a-z]\1").unwrap();
	for line in lines {
		let is_pair = pairs.is_match(&line).unwrap();
		let is_middle = middle.is_match(&line).unwrap();
		if is_pair && is_middle {
			total += 1;
		}
	}
	total
}

fn part1() {
	let lines = read_file("INPUT");
	println!("Part 1: {}", nice_string(&lines));
}

fn part2() {
	let lines = read_file("INPUT");
	println!("Part 2: {}", nicer_string(&lines));
}

fn main() {
	println!("Year 2015 day 5 - Doesn't He Have Intern-Elves For This?");
	part1();
	part2();
}
