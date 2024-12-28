#!/usr/bin/env python3
import re

def read_file(file):
	with open(file, 'r') as f:
		for line in f:
			if len(line) == 0:
				continue
			found = re.search("Valve (\w{2}) has flow rate=(\d+); tunnel[s]* lead[s]* to valve[s]* ([\w,\s]+)", line[:-1])
			valve = found.group(1)
			others = found.group(3)
			number = found.group(2)
			print(f"{number} and {valve} and {others}")

def part1():
	read_file("INPUT")
	print(f"Part 1: {0}")

def part2():
	print(f"Part 2: {0}")

if __name__ == "__main__":
	print("Year 2022 day 16 - Proboscidea Volcanium")
	part1()
	part2()

