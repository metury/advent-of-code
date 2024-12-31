#!/usr/bin/env python3

def read_file(file):
	instructions = ""
	with open(file, 'r') as f:
		for line in f:
			if len(line) == 0:
				continue
			instructions = line[:-1]
	return instructions

def change(block, x, y):
	for i, part in enumerate(block):
			block[i] = (part[0] + x, part[1] + y)

def move(block, movement, blocks):
	if movement == "<":
		change(block, 0, -1)
		for part in block:
			if part[1] < 0:
				change(block, 0, 1)
				break
			for b in blocks:
				if b == part:
					change(block, 0, 1)
					break
	if movement == ">":
		change(block, 0, 1)
		for part in block:
			if part[1] >= 7:
				change(block, 0, -1)
				break
			for b in blocks:
				if b == part:
					change(block, 0, -1)
					break
	change(block, -1, 0)
	for part in block:
		if part[0] < 0:
			change(block, 1, 0)
			return True
		for b in blocks:
			if b == part:
				change(block, 1, 0)
				return True
	return False

class Block:
	def __init__(self):
		self.current = 0
	def new(self, height):
		self.current = (self.current + 1) % 5
		if self.current == 1:
			return [(height, 2), (height, 3), (height, 4), (height, 5)]
		if self.current == 2:
			return [(height+1, 2), (height+1, 3), (height+1, 4), (height+2, 3), (height, 3)]
		if self.current == 3:
			return [(height, 2), (height, 3), (height, 4), (height+1, 4), (height+2, 4)]
		if self.current == 4:
			return [(height, 2), (height+1, 2), (height+2, 2), (height+3, 2)]
		if self.current == 0:
			return [(height+1, 2), (height+1, 3), (height, 2), (height, 3)]

def part1():
	instructions = read_file("INPUT")
	block = Block()
	blocks = []
	height = 3
	current = block.new(height)
	counter = 0
	i = 0
	while counter < 2022:
		end = move(current, instructions[i], blocks)
		i = (i + 1) % len(instructions)
		if end:
			counter += 1
			for c in current:
				blocks.append(c)
			height = max([b[0] for b in blocks]) + 4
			current = block.new(height)
	print(f"Part 1: {max([b[0] for b in blocks]) + 1}")

def part2():
	instructions = read_file("INPUT")
	block = Block()
	blocks = []
	height = 3
	current = block.new(height)
	counter = 0
	i = 0
	while counter < 1000000000000:
		end = move(current, instructions[i], blocks)
		i = (i + 1) % len(instructions)
		if end:
			counter += 1
			for c in current:
				blocks.append(c)
			height = max([b[0] for b in blocks]) + 4
			current = block.new(height)
	print(f"Part 1: {max([b[0] for b in blocks]) + 1}")

if __name__ == "__main__":
	print("Year 2022 day 17 - Pyroclastic Flow")
	part1()
	part2()

