use std::fs;
use z3::ast::{Ast, Int};
use z3::{Config, Context, Solver};

struct HailStone {
	x: i64,
	y: i64,
	z: i64,
	xv: i64,
	yv: i64,
	zv: i64,
}

fn parse_hail_stone(line: &str) -> HailStone {
	let pos_vel: Vec<&str> = line.split(" @ ").collect();
	let pos: Vec<i64> = pos_vel[0].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap()).collect();
	let vel: Vec<i64> = pos_vel[1].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap()).collect();
	HailStone {
		x: pos[0],
		y: pos[1],
		z: pos[2],
		xv: vel[0],
		yv: vel[1],
		zv: vel[2],
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

fn colission(hs1: &HailStone, hs2: &HailStone, limits: (i64, i64)) -> bool {
	let det = (-hs1.xv) * hs2.yv - hs2.xv * (-hs1.yv);
	if det == 0 {
		return false;
	} else {
		let time1 = ((hs1.x - hs2.x) * hs2.yv - (hs1.y - hs2.y) * hs2.xv) / det;
		let time2 = ((-hs1.xv) * (hs1.y - hs2.y) - (-hs1.yv) * (hs1.x - hs2.x)) / det;
		let point = (hs1.x + hs1.xv * time1, hs1.y + hs1.yv * time1);
		return point.0 >= limits.0 && point.0 <= limits.1 && point.1 >= limits.0 && point.1 <= limits.1 && time1 >= 0 && time2 >= 0;
	}
}

fn number_of_collisions(hail_stones: &Vec<HailStone>, limits: (i64, i64)) -> u64 {
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

fn perfect_shot(hailstones: &Vec<HailStone>) -> i64{
	let cfg = Config::new();
	let ctx = Context::new(&cfg);
	let solver = Solver::new(&ctx);
	let px = Int::new_const(&ctx, "px");
	let py = Int::new_const(&ctx, "py");
	let pz = Int::new_const(&ctx, "pz");
	let vx = Int::new_const(&ctx, "vx");
	let vy = Int::new_const(&ctx, "vy");
	let vz = Int::new_const(&ctx, "vz");
	for hailstone in hailstones {
		let pxn = Int::from_i64(&ctx, hailstone.x);
		let pyn = Int::from_i64(&ctx, hailstone.y);
		let pzn = Int::from_i64(&ctx, hailstone.z);
		let vxn = Int::from_i64(&ctx, hailstone.xv);
		let vyn = Int::from_i64(&ctx, hailstone.yv);
		let vzn = Int::from_i64(&ctx, hailstone.zv);
		let tn = Int::fresh_const(&ctx, "t");
		solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
		solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
		solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
	}
	solver.check();
	let model = solver.get_model().unwrap();
	let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
	let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
	let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();
	x + y + z
}

fn part1() {
	let hail_stones = read_file("INPUT");
	println!("Part 1: {}", number_of_collisions(&hail_stones, (200_000_000_000_000, 400_000_000_000_000)));
}

fn part2() {
	let hail_stones = read_file("INPUT");
	println!("Part 2: {}", perfect_shot(&hail_stones));
}

fn main() {
	println!("Year 2023 day 24 - Never Tell Me The Odds");
	part1();
	part2();
}
