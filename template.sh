#!/bin/bash

set -ueo pipefail

if [ $# -lt 1 ]; then
	echo "You must provide a name as an argument."
	exit
fi

day=$(date +'%d')
year=$(date +'%Y')
name=$1

if [ $# -gt 2 ]; then
	day=$3
	year=$2
fi

day=$(printf "%02d" $day)

mkdir -p "$year"
cd "$year"

if [ -d "$day" ]; then
	echo "The directory $day in the year $year already exists. Before running this script ensure it does not exist."
	exit
fi

cargo new "aoc-$year-$day"
mv "aoc-$year-$day" "$day"
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
	println!(\"Year $year day $day - $name\");
	part1();
	part2();
}" > main.rs

cd ..

touch "INPUT"
touch "info.md"

echo "#### Part 1

#### Part 2
" > "info.md"

cd ..
