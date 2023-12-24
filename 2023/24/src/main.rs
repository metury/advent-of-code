use std::fs;

#[derive(Debug)]
struct HailStone {
	x: i64,
	y: i64,
	z: i64,
	x_shift: i64,
	y_shift: i64,
	z_shift: i64,
}

fn parse_hail_stone(line: &str) -> HailStone {
	let pos_vel: Vec<&str> = line.split(" @ ").collect();
	let pos: Vec<i64> = pos_vel[0].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap()).collect();
	let vel: Vec<i64> = pos_vel[1].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap()).collect();
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
	// Matrix a1 a2 = c1
	//        b1 b2 = c2
	let a1 = -hs1.x_shift as f64;
	let a2 = hs2.x_shift as f64;
	let b1 = -hs1.y_shift as f64;
	let b2 = hs2.y_shift as f64;
	let c1 = (hs1.x - hs2.x) as f64;
	let c2 = (hs1.y - hs2.y) as f64;
	let det = a1 * b2 - a2 * b1;
	if det == 0.0 {
		return false;
	} else {
		let s = (c1 * b2 - c2 * a2) / det;
		let t = (a1 * c2 - b1 * c1) / det;
		let point = (hs1.x as f64 + hs1.x_shift as f64 * s, hs1.y as f64 + hs1.y_shift as f64 * s);
		return point.0 >= limits.0 && point.0 <= limits.1 && point.1 >= limits.0 && point.1 <= limits.1 && s >= 0.0 && t >= 0.0;
	}
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
	println!("Part 1: {}", number_of_collisions(&hail_stones, (200_000_000_000_000.0, 400_000_000_000_000.0)));
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 24 - Never Tell Me The Odds");
	part1();
	part2();
}
