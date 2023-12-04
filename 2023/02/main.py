#!/usr/bin/env python3

prefix = "Game "
colors = ["red", "blue", "green"]
limits = [12, 14, 13]
first_sum = 0
second_sum = 0

def under_limits(counters, limits):
	for i in range(len(counters)):
		if limits[i] < counters[i]:
			return False
	return True

with open('INPUT') as f:
	for line in f:
		counters =[0 for _ in colors]
		game = line[:-1].split(": ")
		nr_game = int(game[0][len(prefix):])
		sep = [draw.split(", ") for draw in game[1].split("; ")]
		nr_co = [[y.split(" ") for y in x] for x in sep]
		for i in range(len(nr_co)):
			for j in range(len(nr_co[i])):
				nr_co[i][j][0] = int(nr_co[i][j][0])
				counters[colors.index(nr_co[i][j][1])] = max(nr_co[i][j][0], counters[colors.index(nr_co[i][j][1])])
		if under_limits(counters, limits):
			first_sum += nr_game
		second_sum += counters[0] * counters[1] * counters[2]

print(f"First part: {first_sum}")
print(f"Second part: {second_sum}")
