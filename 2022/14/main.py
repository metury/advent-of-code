#!/usr/bin/env python3

def read(s):
	tmp = s.split(' -> ')
	ret = []
	for t in tmp:
		nr = t.split(',')
		ret.append((int(nr[0]), int(nr[1])))
	return ret

d = set()

def addd(r, lo):
	index = 0
	while index+1 < len(r):
		f = r[index]
		s = r[index+1]
		index += 1
		for i in range(min(f[0], s[0]), max(f[0],s[0])+1):
			for j in range(min(f[1], s[1]), max(f[1],s[1])+1):
				d.add((i,j))
				if j > lo:
					lo = j
	return lo

def sand(lo):
	s = (500, 0)
	while s[1] <= lo:
		if (s[0], s[1]+1) in d:
			if (s[0]-1, s[1] + 1) in d:
				if (s[0] + 1, s[1] + 1) in d:
					d.add(s)
					return 1
				s = (s[0] + 1, s[1] + 1)
				continue
			s = (s[0] - 1, s[1] + 1)
			continue
		s = (s[0], s[1] + 1)
	return 0

def sand2(lo):
	s = (500, 0)
	if s in d:
		return 0
	floor = lo + 2
	while s[1] < floor:
		if (s[0], s[1]+1) in d or s[1]+1 == floor:
			if (s[0]-1, s[1] + 1) in d or s[1] +1  == floor:
				if (s[0] + 1, s[1] + 1) in d or s[1] +1 == floor:
					d.add(s)
					return 1
				s = (s[0] + 1, s[1] + 1)
				continue
			s = (s[0] - 1, s[1] + 1)
			continue
		s = (s[0], s[1] + 1)
	return 0

with open("INPUT") as f:
	lo = 0
	summ = 0
	for line in f:
		r = read(line[:-1])
		lo = addd(r, lo)
	while (sand(lo) == 1):
		summ += 1
	print(f"Prvni: {summ}")
	while sand2(lo) == 1:
		summ += 1
	print(f"Druhy: {summ}")
