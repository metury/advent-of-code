#!/usr/bin/env python3
from functools import cmp_to_key

def read(string):
	inp = []
	number = 0
	was_number = False
	i = 0
	while i < len(string):
		c = string[i]
		if c == '[':
			np, index = read(string[i+1:])
			was_number = False
			inp.append(np)
			i += index + 1
		elif c == ']':
			if was_number:
				inp.append(number)
			return inp, i
		elif c == ',':
			if was_number:
				inp.append(number)
			number = 0
			was_number = False
		else:
			was_number = True
			number *= 10
			number += int(c)
		i += 1
	return np

def compare(x,y):
	if type(x) != type(y):
		return compare([x],y) if type(x) is int else compare(x,[y])
	else:
		if type(x) is int:
			if x != y:
				return -1 if x > y else 1
			return 0
		else:
			for i in range(min(len(x), len(y))):
				if (res := compare(x[i], y[i])) != 0:
					return res
			if len(x) != len(y):
				return -1 if len(x) > len(y) else 1
			return 0

list_all = [[[2]],[[6]]]
m = 1

with open("INPUT") as f:
	left = []
	right = []
	index = 0
	iSum = 0
	for line in f:
		if line == '\n':
			index += 1
			tmp = compare(left,right)
			if tmp != -1:
				iSum += index
		else:
			left = right
			right = read(line[:-1])
			list_all.append(right)
	print(f"Prvni {iSum}")
	s = sorted(list_all, key=cmp_to_key(lambda x, y : -compare(x,y)))
	for i in range(len(s)):
		if s[i] == [[2]]:
			m *= (i+1)
		if s[i] == [[6]]:
			m *= (i+1)
			print(f"Druhy: {m}")
			break
