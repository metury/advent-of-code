#/bin/bash

set -ueo pipefail

year=$1
day=$2
name=$3

mkdir -p "$year"
cd "$year"
cargo new "rusty-day"
mv "rusty-day" "$day"
cd "$day"/src/
echo "use std::fs;

fn read_file(filepath: &str) -> {
	let contents = fs::read_to_string(filepath);
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		for c in line.chars(){
		}
	}
	return;
}

fn part1(){
	println!("Part 1: {}", 0);
}

fn part2(){
	println!("Part 2: {}", 0);
}

fn main() {
	println!("Day {} - {}", $day, "$name");
	part1();
	part2();
}" > main.rs

cd ..
touch "INPUT"
