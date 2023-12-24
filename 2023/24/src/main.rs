use std::fs;
use good_lp::{variables, variable, default_solver, SolverModel, Solution};

#[derive(Debug)]
struct HailStone {
	x: f64,
	y: f64,
	z: f64,
	x_shift: f64,
	y_shift: f64,
	z_shift: f64,
}

fn parse_hail_stone(line: &str) -> HailStone {
	let pos_vel: Vec<&str> = line.split(" @ ").collect();
	let pos: Vec<f64> = pos_vel[0].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap() as f64).collect();
	let vel: Vec<f64> = pos_vel[1].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap() as f64).collect();
	HailStone {
		x: pos[0],
		y: pos[1],
		z: pos[2],
		x_shift: vel[0],
		y_shift: vel[1],
		z_shift: vel[2],
	}
}

fn read_file(filepath: &str) -> Vec<HailStone> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	let mut hail_stones: Vec<HailStone> = vec!();
	for line in lines{
		if line != "" {
			hail_stones.push(parse_hail_stone(line));
		}
	}
	hail_stones
}

fn colission(hs1: &HailStone, hs2: &HailStone, limits: (f64, f64)) -> bool {
	variables!{
		vars:
			t;
			7 <= x <= 27;
			7 <= y <= 27;
	}
	let solution = vars.maximise(t)
		.using(default_solver)
		.with(x << hs1.x_shift + t * hs1.x_shift )
		.with(hs1.x_shift + t * hs1.x_shift << x)
		.with(x << hs2.x_shift + t * hs2.x_shift )
		.with(hs2.x_shift + t * hs2.x_shift << x)
		.with(y << hs1.y_shift + t * hs1.y_shift )
		.with(hs2.y_shift + t * hs2.y_shift << y)
		.solve();
	match solution {
		Ok (solution) => println!("{} {}", solution.value(x), solution.value(y)),
		Err (..) => return false,
	}
	true
}

fn number_of_collisions(hail_stones: &Vec<HailStone>, limits: (f64, f64)) -> u64 {
	let mut total: u64 = 0;
	for i in 0 .. hail_stones.len() {
		for j in i + 1 .. hail_stones.len() {
			if colission(&hail_stones[i], &hail_stones[j], limits) {
				total += 1;
			}
		}
	}
	total
}

fn part1() {
	let hail_stones = read_file("INPUT");
	println!("Part 1: {}", number_of_collisions(&hail_stones, (7.0, 27.0)));
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 24 - Never Tell Me The Odds");
	part1();
	part2();
}
