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
mkdir -p "$day"
cd "$day"

echo "#!/usr/bin/env python3

def read_file(filepath):
	with open(filepath, 'r') as f:
		for line in f:
			print(line)

def part1():
	print(f\"Part 1: {0}\")

def part2():
	print(f\"Part 2: {0}\")

if __name__ == \"__main__\":
	print(\"Year $year day $day - $name\")
	part1()
	part2()" > main.py

chmod +x main.py

cd ../..

./general.sh "$year" "$day"
