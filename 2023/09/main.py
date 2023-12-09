#!/usr/bin/env python3

first_sum = 0
second_sum = 0

def all_zero(array):
	return all([x == 0 for x in array])

def diffs(array):
	new_array = []
	for i in range(len(array) - 1):
		new_array.append(array[i+1] - array[i])
	return new_array

def create_history(array):
	hist = [array]
	while not all_zero(hist[-1]):
		hist.append(diffs(hist[-1]))
	return hist

def propagate(hist):
	hist[-1].append(0)
	for i in range(1, len(hist)):
		size = len(hist) - i - 1
		hist[size].append(hist[size + 1][-1] + hist[size][-1])
	return hist

def propagate_first(hist):
	hist[-1].insert(0, 0)
	for i in range(1, len(hist)):
		size = len(hist) - i - 1
		hist[size].insert(0, hist[size][0] - hist[size + 1][0])

with open('INPUT') as f:
	for line in f:
		hist = create_history([int(x) for x in line.strip().split(" ")])
		propagate(hist)
		propagate_first(hist)
		first_sum += hist[0][-1]
		second_sum += hist[0][0]

print(f"First part: {first_sum}")
print(f"Second part: {second_sum}")
