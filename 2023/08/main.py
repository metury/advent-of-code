#!/usr/bin/env python3

""" Initialization. """
moves = ""
mapping = dict()
move = {'L' : 0, 'R' : 1}
starting_nodes = list()
ending_nodes = list()
start = 'AAA'
end = 'ZZZ'

""" Read input. """
with open('INPUT') as f:
	for line in f:
		if len(line.strip()) != 0:
			if moves == "":
				moves = line.strip()
			else:
				from_to = line.strip().split(" = ")
				mapping[from_to[0]] = from_to[1][1:-1].split(", ")
				if from_to[0][-1] == 'A':
					starting_nodes.append(from_to[0])
				elif from_to[0][-1] == 'Z':
					ending_nodes.append(from_to[0])

def find(moves, mapping, move, point, end):
	""" Find path from point to some point in the end.
	    Using moves and given mapping. """
	counter = 0
	while True:
		for c in moves:
			point = mapping[point][move[c]]
			counter += 1
			if point in end:
				return counter

def gcd(a, b):
	""" Greatest common denominator. """
	if a < b:
		a, b = b, a
	while b != 0:
		t = b
		b = a % b
		a = t
	return a

def lcm(a, b):
	""" Least common multiple. """
	return (a * b) / gcd(a, b)

def lcm_list(lengths):
	""" Least common multiple for more numbers. """
	numbers = [j for j in lengths]
	while len(numbers) > 1:
		first, second = numbers[0], numbers[1]
		numbers.pop(0)
		numbers.pop(0)
		numbers.append(lcm(first, second))
	return int(numbers[0])

def find_common(moves, mapping, move, starting_nodes, ending_nodes):
	""" Find common number for multiple paths. """
	lengths = set()
	for point in starting_nodes:
		lengths.add(find(moves, mapping, move, point, ending_nodes))
	return lcm_list(lengths)

print(f"First part: {find(moves, mapping, move, start, [end])}")
print(f"Second part: {find_common(moves, mapping, move, starting_nodes, ending_nodes)}")
