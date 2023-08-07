#!/usr/bin/env python3

total_sum = 0
overlap_sum = 0

with open("INPUT") as f:
	for line in f:
		if line != '\n':
			comp = line.split(",")
			first = comp[0].split("-")
			second = comp[1].split("-")
			diff1 = int(first[0]) - int(second[0])
			diff2 = int(first[1]) - int(second[1])
			overlap1 = int(first[1]) - int(second[0])
			overlap2 = int(first[0]) - int(second[1])
			if (diff1 <= 0 and diff2 >= 0) or (diff1 >= 0 and diff2 <= 0):
				total_sum += 1
			if (overlap1 == 0 or overlap2 == 0) or (overlap1 > 0 and overlap2 < 0) or (overlap1 < 0 and overlap2 > 0):
				overlap_sum += 1

print(f"Prvni: {total_sum}")
print(f"Druhy: {overlap_sum}")
