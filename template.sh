#!/bin/bash

set -ueo pipefail

day=$(date +'%d')
year=$(date +'%Y')

if [ $# -gt 1 ]; then
	year=$1
	day=$2
fi

file=$(mktemp)
wget "https://adventofcode.com/$year/day/$day" -O "$file"
title=$(grep -o -e "--- Day [0-9]*: .* ---" "$file")
tmp=${title#*: }
name=${tmp% ---*}
echo "$name"
echo "DD"
rm "$file"

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

info="info.md"
touch "INPUT"
touch "$info"

echo "#### Part 1

#### Part 2
" > "$info"

cd ..
