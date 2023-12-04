#!/usr/bin/env python3

first_sum = 0
second_sum = 0

with open('INPUT') as f:
	multiples = [1]
	for line in f:
		if len(multiples) == 0:
			multiples.append(1)
		hand = line.strip().split(":")
		types = hand[1].strip().split("|")
		winning = types[0].strip().split(" ")
		winning = filter(lambda nr : len(nr) > 0, winning)
		win_set = set([int(w) for w in winning])
		having = types[1].strip().split(" ")
		having = filter(lambda nr : len(nr) > 0, having)
		hav_set = set([int(h) for h in having])
		res = win_set.intersection(hav_set)
		for i in range(len(res)):
			if i + 1 >= len(multiples):
				multiples.append(1)
			multiples[i+1] += multiples[0]
		second_sum += multiples[0]
		multiples.pop(0)
		points = 0
		if len(res) > 0:
			points = 2 ** (len(res) - 1)
		first_sum += points

print(f"First part: {first_sum}")
print(f"Second part: {second_sum}")
