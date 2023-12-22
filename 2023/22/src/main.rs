use std::fs;
use std::collections::HashSet;

#[derive(Debug)]
struct Brick {
	x: (i64, i64),
	y: (i64, i64),
	z: (i64, i64),
	supports: HashSet<usize>,
	supported_by: HashSet<usize>,
}

fn parse_numbers(line: &str) -> (i64, i64, i64) {
	let parts: Vec<&str> = line.split(',').into_iter().collect();
	(i64::from_str_radix(&parts[0], 10).unwrap(),
	 i64::from_str_radix(&parts[1], 10).unwrap(),
	 i64::from_str_radix(&parts[2], 10).unwrap())
}

fn parse_brick(line: &str) -> Brick {
	let parts: Vec<&str> = line.split('~').into_iter().collect();
	let (x1, y1, z1) = parse_numbers(parts[0]);
	let (x2, y2, z2) = parse_numbers(parts[1]);
	Brick { x: (x1, x2),
		    y: (y1, y2),
		    z: (z1, z2),
		    supported_by: HashSet::new(),
		    supports: HashSet::new()}
}

fn read_file(filepath: &str) -> Vec<Brick> {
	let contents = fs::read_to_string(filepath);
	let mut bricks: Vec<Brick> = vec!();
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		if line != "" {
			bricks.push(parse_brick(line));
		}
	}
	bricks
}

fn overlap(b1: &Brick, b2: &Brick) -> bool {
	b1.x.0 <= b2.x.1 && b1.x.1 >= b2.x.0 && b1.y.0 <= b2.y.1 && b1.y.1 >= b2.y.0
}

fn compress_bricks(bricks: &mut Vec<Brick>) {
	for i in 0 .. bricks.len() {
		let mut max: i64 = 0;
		for j in 0 .. i {
			if bricks[j].z.1 > max && overlap(&bricks[i], &bricks[j]) {
				max = bricks[j].z.1;
			}
		}
		max += 1;
		bricks[i].z.1 = bricks[i].z.1 - bricks[i].z.0 + max;
		bricks[i].z.0 = max;
	}
}

fn count_overlaps(bricks: &mut Vec<Brick>) -> usize {
	let mut set: HashSet<usize> = HashSet::new();
	for i in 0 .. bricks.len() {
		for j in 0 .. bricks.len() {
			if bricks[j].z.1 == bricks[i].z.0 - 1 && overlap(&bricks[i], &bricks[j]) {
				bricks[i].supported_by.insert(j);
			}
			if bricks[i].z.1 + 1 == bricks[j].z.0 && overlap(&bricks[i], &bricks[j]) {
				bricks[i].supports.insert(j);
			}
		}
		if bricks[i].supported_by.len() > 1 {
			for s in &bricks[i].supported_by {
				set.insert(*s);
			}
		}
		if bricks[i].supports.len() == 0 {
			set.insert(i);
		}
	}
	set.len()
}

fn part1() {
	let mut bricks = read_file("INPUT");
	bricks.sort_by(|a, b| a.z.cmp(&b.z));
	compress_bricks(&mut bricks);
	println!("Part 1: {}", count_overlaps(&mut bricks));
}

fn part2() {
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Year 2023 day 22 - Sand Slabs");
	part1();
	part2();
}
