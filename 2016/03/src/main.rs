use std::fs;
use regex::Regex;

struct Triangle {
	a: i64,
	b: i64,
	c: i64,
}

impl Triangle {
	fn new(a: i64, b: i64, c: i64) -> Self {
		Triangle {a: a, b: b, c: c}
	}
	fn is_valid(&self) -> bool {
		(self.a + self.b > self.c) && (self.b + self.c > self.a) && (self.c + self.a > self.b)
	}
}

fn read_file(filepath: &str) -> Vec<Triangle> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let re = Regex::new(r"([0-9]+)\s+([0-9]+)\s+([0-9]+)").unwrap();
	let mut triangles: Vec<Triangle> = vec!();
	for (_, [a, b, c]) in re.captures_iter(&binding).map(|c| c.extract()) {
		triangles.push(Triangle::new(a.parse().unwrap(), b.parse().unwrap(), c.parse().unwrap()));
	}
	triangles
}

fn convert_triangles(triangles: &Vec<Triangle>) -> Vec<Triangle> {
	let mut column_triangles: Vec<Triangle> = vec!();
	for (i,_) in triangles.iter().enumerate().step_by(3) {
		column_triangles.push(Triangle::new(triangles[i].a, triangles[i + 1].a, triangles[i + 2].a));
		column_triangles.push(Triangle::new(triangles[i].b, triangles[i + 1].b, triangles[i + 2].b));
		column_triangles.push(Triangle::new(triangles[i].c, triangles[i + 1].c, triangles[i + 2].c));
	}
	column_triangles
}

fn part1() {
	let triangles = read_file("INPUT");
	let res = triangles.into_iter().map(|t| if t.is_valid() {1} else {0}).sum::<i64>();
	println!("Part 1: {}", res);
}

fn part2() {
	let triangles = convert_triangles(&read_file("INPUT"));
	let res = triangles.into_iter().map(|t| if t.is_valid() {1} else {0}).sum::<i64>();
	println!("Part 2: {}", res);
}

fn main() {
	println!("Year 2016 day 3 - Squares With Three Sides");
	part1();
	part2();
}
