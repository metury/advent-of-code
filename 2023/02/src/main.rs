use std::fs;
use regex::Regex;
use std::cmp::max;

type Colors = (i64, i64, i64);

fn read_file(filepath: &str) -> Vec<String> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines: Vec<String> = binding.split('\n')
			.filter(|c| c.len() > 0)
			.map(|c| c.to_string()).collect();
	lines
}

fn get_colors(line: &str) -> (i64, Vec<Colors>) {
	let re_game = Regex::new(r"Game ([0-9]+)").unwrap();
	let game = re_game.captures(line).unwrap()[1].parse::<i64>().unwrap();
	let mut colors: Vec<Colors> = vec!();
	let re_red = Regex::new(r"([0-9]+) red").unwrap();
	let re_blue = Regex::new(r"([0-9]+) blue").unwrap();
	let re_green = Regex::new(r"([0-9]+) green").unwrap();
	for part in line.split(';') {
		let mut red = 0_i64;
		for (_, [number]) in re_red.captures_iter(part).map(|c| c.extract()) {
			red += number.parse::<i64>().unwrap();
		}
		let mut blue = 0_i64;
		for (_, [number]) in re_blue.captures_iter(part).map(|c| c.extract()) {
			blue += number.parse::<i64>().unwrap();
		}
		let mut green = 0_i64;
		for (_, [number]) in re_green.captures_iter(part).map(|c| c.extract()) {
			green += number.parse::<i64>().unwrap();
		}
		colors.push((red, green, blue));
	}
	(game, colors)
}

fn part1() {
	let lines = read_file("INPUT");
	let limits: [i64; 3] = [12, 13, 14];
	let mut total: i64 = 0;
	for line in lines {
		let (game, colors) = get_colors(&line);
		let feasible: bool = colors.into_iter()
				.map(|(r, g, b)| r <= limits[0] && g <= limits[1] && b <= limits[2])
				.fold(true, |acc, x| acc && x);
		if feasible {
			total += game;
		}
	}
	println!("Part 1: {}", total);
}

fn part2() {
	let lines = read_file("INPUT");
	let mut total: i64 = 0;
	for line in lines {
		let (_game, colors) = get_colors(&line);
		let maxes = colors.into_iter()
			.fold((0,0,0), |acc, (r, g, b)| (max(acc.0, r), max(acc.1, g), max(acc.2, b)));
		total += maxes.0 * maxes.1 * maxes.2;
	}
	println!("Part 2: {}", total);
}

fn main() {
	println!("Year 2023 day 2 - Cube Conundrum");
	part1();
	part2();
}
