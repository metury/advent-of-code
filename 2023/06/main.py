#!/usr/bin/env python3

times = []
records = []
tot_time = 0
tot_record = 0

first_sum = 1
second_sum = 0

def ignore_space(string):
	num = 0
	for c in string:
		if c.isdigit():
			num = num * 10 + int(c)
	return num

with open('INPUT') as f:
	for line in f:
		parts = list(filter(lambda part : len(part) > 0, line.strip().split(" ")))
		splitted_line = line.strip().split(":")
		if splitted_line[0] == "Time":
			times = [int(x) for x in parts[1:]]
			tot_time = ignore_space(splitted_line[1].strip())
		else:
			records = [int(x) for x in parts[1:]]
			tot_record = ignore_space(splitted_line[1].strip())

for i in range(len(times)):
	counter = 0
	for t in range(times[i]):
		speed = t
		remaining_time = times[i] - t
		travel = speed * remaining_time
		if travel > records[i]:
			counter += 1
	first_sum *= counter


"""
This may not be fast, but it will do.
"""

for t in range(tot_time):
	speed = t
	remaining_time = tot_time - t
	travel = speed * remaining_time
	if travel > tot_record:
		second_sum += 1

print(f"First part: {first_sum}")
print(f"Second sum: {second_sum}")
