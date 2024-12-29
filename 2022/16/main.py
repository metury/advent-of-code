#!/usr/bin/env python3
import re

class Valve:
	def __init__(self, name, number, others):
		self.name = name
		self.number = number
		self.others = others
		self.opened = False

def read_file(file):
	d = dict()
	with open(file, 'r') as f:
		for line in f:
			if len(line) == 0:
				continue
			found = re.search("Valve (\w{2}) has flow rate=(\d+); tunnels* leads* to valves* ([\w,\s]+)", line[:-1])
			valve = found.group(1)
			others = found.group(3).split(", ")
			number = found.group(2)
			d[valve] = Valve(valve, int(number), others)
	return d

# Floyd-Warshall algorithm
def distances(valves, indices):
	matrix = list()
	for i in range(len(indices)):
		matrix.append(list())
		for j in indices:
			matrix[i].append(len(indices) + 1)
	for key in valves:
		i = indices.index(key)
		for other in valves[key].others:
			j = indices.index(other)
			matrix[i][j] = 1
			matrix[j][i] = 1
	for k in range(len(indices)):
		for i in range(len(indices)):
			for j in range(len(indices)):
				matrix[i][j] = min(matrix[i][j], matrix[i][k] + matrix[k][j])
	return matrix

def maximize(valves, dist_matrix, non_empty, indices, current, summ, flow_rate, time):
	if time < 0:
		return 0
	result = summ
	for other in non_empty:
		i = indices.index(current)
		j = indices.index(other)
		dist = dist_matrix[i][j]
		if not valves[other].opened and time - dist - 1 >= 0:
			valves[other].opened = True
			result = max(result, maximize(valves, dist_matrix, non_empty, indices, other, summ + ((dist + 1) * flow_rate), flow_rate + valves[other].number, time - dist - 1))
			valves[other].opened = False
		if time > 0:
			result = max(result, summ + time * flow_rate)
	return result

def part1():
	valves = read_file("INPUT")
	indices = list()
	non_empty = list()
	for key in valves:
		indices.append(key)
		if valves[key].number > 0:
			non_empty.append(key)
	dist_matrix = distances(valves, indices)
	result = maximize(valves, dist_matrix, non_empty, indices, "AA", 0, 0, 30)
	print(f"Part 1: {result}")

def part2():
	print(f"Part 2: {0}")

if __name__ == "__main__":
	print("Year 2022 day 16 - Proboscidea Volcanium")
	part1()
	part2()

