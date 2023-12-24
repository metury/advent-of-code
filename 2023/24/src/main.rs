use std::fs;

const EPSILON: f64 = 0.0001;

struct HailStone {
	x: f64,
	y: f64,
	z: f64,
	xv: f64,
	yv: f64,
	zv: f64,
}

fn parse_hail_stone(line: &str) -> HailStone {
	let pos_vel: Vec<&str> = line.split(" @ ").collect();
	let pos: Vec<f64> = pos_vel[0].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap() as f64).collect();
	let vel: Vec<f64> = pos_vel[1].split(", ").map(|x| i64::from_str_radix(x, 10).unwrap() as f64).collect();
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

fn colission(hs1: &HailStone, hs2: &HailStone, limits: (f64, f64)) -> bool {
	let det = (-hs1.xv) * hs2.yv - hs2.xv * (-hs1.yv);
	if det == 0.0 {
		return false;
	} else {
		let time1 = ((hs1.x - hs2.x) * hs2.yv - (hs1.y - hs2.y) * hs2.xv) / det;
		let time2 = ((-hs1.xv) * (hs1.y - hs2.y) - (-hs1.yv) * (hs1.x - hs2.x)) / det;
		let point = (hs1.x + hs1.xv * time1, hs1.y + hs1.yv * time1);
		return point.0 >= limits.0 && point.0 <= limits.1 && point.1 >= limits.0 && point.1 <= limits.1 && time1 >= 0.0 && time2 >= 0.0;
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


fn solve_part2(h1: &HailStone, h2: &HailStone, vx: f64, vy: f64, vz: f64) -> Option<(f64, f64, (f64, f64, f64))> {
	let t2_numerator = h2.y - h1.y - (((h1.yv - vy) * (h2.x - h1.x)) / (h1.xv - vx));
	let t2_denominator = vy - h2.yv - (((h1.yv - vy) * (vx - h2.xv)) / (h1.xv - vx));
	let t2 = t2_numerator / t2_denominator;
	let t1 = (h2.x - h1.x - t2 * (vx - h2.xv)) / (h1.xv - vx);
	let px = h1.x - t1 * (vx - h1.xv);
	let py = h1.y - t1 * (vy - h1.yv);
	let pz = h1.z - t1 * (vz - h1.zv);
	if (pz + t2 * (vz - h2.zv) - h2.z).abs() > EPSILON {
		None
	} else {
		Some((t1, t2,(px, py, pz)))
	}
}
fn perfect_shot(hail_stones: &Vec<HailStone>) -> usize {
	let a = &hail_stones[0];
	let b = &hail_stones[1];
	let is_int = |f: f64| (f.round() - f).abs() < EPSILON;
	for vx in -500..500 {
		for vy in -500..500 {
			'outer: for vz in -500..500 {
				let vx = vx as f64;
				let vy = vy as f64;
				let vz = vz as f64;
				if let Some((t1, t2, (px, py, pz))) = solve_part2(&a, &b, vx, vy, vz) {
					if (!(t1.is_finite() && t2.is_finite() && px.is_finite() && py.is_finite() && pz.is_finite()))
					   || (t1.is_sign_negative() || t2.is_sign_negative())
					   || (!(is_int(t1) && is_int(t2) && is_int(px) && is_int(py) && is_int(pz))){
						continue;
					}
					for i in 2 .. hail_stones.len() {
						let h = &hail_stones[i];
						let t3 = (h.x - px) / (vx - h.xv);
						if (py + t3 * vy - (h.y + t3 * h.yv)).abs() > EPSILON
							|| (pz + t3 * vz - (h.z + t3 * h.zv)).abs() > EPSILON {
							continue 'outer;
						}
						return px as usize + py as usize + pz as usize;
					}
				}
			}
		}
	}
	404
}

fn part1() {
	let hail_stones = read_file("INPUT");
	println!("Part 1: {}", number_of_collisions(&hail_stones, (200_000_000_000_000.0, 400_000_000_000_000.0)));
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
