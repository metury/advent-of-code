#!/usr/bin/env python3

original_map = []

def find_galaxies(original_map):
	galaxies = []
	for i in range(len(original_map)):
		for j in range(len(original_map[i])):
			if original_map[i][j] == "#":
				galaxies.append((i,j))
	return galaxies

def empty_list(array):
	return all([x == "." for x in array])

def empty(original_map):
	rows = []
	columns = []
	for i in range(len(original_map)):
		if empty_list(original_map[i]):
			columns.append(i)
	for j in range(len(original_map[0])):
		if empty_list([original_map[x][j] for x in range(len(original_map))]):
			rows.append(j)
	return rows, columns

def expand(gal, rows, columns, addition):
	for i in range(len(gal)):
		add = 0
		original = True
		for r in rows:
			if r < gal[i][1]:
				add += addition
			else:
				gal[i] = (gal[i][0], gal[i][1] + add)
				original = False
				break
		if original:
			gal[i] = (gal[i][0], gal[i][1] + add)
		add = 0
		original = True
		for c in columns:
			if c < gal[i][0]:
				add += addition
			else:
				gal[i] = (gal[i][0] + add, gal[i][1])
				original = False
				break
		if original:
			gal[i] = (gal[i][0] + add, gal[i][1])

def length(gal):
	total = 0
	for i in range(len(gal)):
		for j in range(i + 1, len(gal)):
			dist = abs(gal[i][0] - gal[j][0]) + abs(gal[i][1] - gal[j][1])
			total += dist
	return total

with open('INPUT') as f:
	for line in f:
		original_map.append([x for x in line.strip()])

galaxies = find_galaxies(original_map)
rows, columns = empty(original_map)
expand(galaxies, rows, columns, 2 - 1)
print(f"First part: {length(galaxies)}")
galaxies = find_galaxies(original_map)
expand(galaxies, rows, columns, 1_000_000 - 1)
print(f"Second part: {length(galaxies)}")
