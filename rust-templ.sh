#/bin/bash

# Create a templated rust project for given problem.
# There are three arguments:
#  1 Year
#  2 Day
#  3 Name

set -ueo pipefail

if [ $# -lt 2 ]; then
	echo "Three arguments were not found. Year, day and name must be provided."
	exit
fi

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
	let binding = contents.expect(\"REASON\");
	let lines = binding.split('\n');
	for line in lines{
		for c in line.chars(){
		}
	}
	return;
}

fn part1(){
	println!(\"Part 1: {}\", 0);
}

fn part2(){
	println!(\"Part 2: {}\", 0);
}

fn main() {
	println!(\"Year {} day {} - {}\", $year, $day, $name);
	part1();
	part2();
}" > main.rs

cd ../../..

./general.sh "$year" "$day"
