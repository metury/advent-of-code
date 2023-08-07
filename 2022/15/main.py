#!/usr/bin/env python3

# Solution is relatively slow

class Sensor:
	def __init__(self, x, y, bx, by):
		self.x = x
		self.y = y
		self.bx = bx
		self.by = by
	def map(self, m, minx, maxx, addd, index):
		for i in range(len(m)):
			if abs(self.y - (index)) + abs(self.x - (i + minx)) <= self.length():
				m[i] = True
	def map2(self, i, j):
		if abs(self.x - i) + abs(self.y - j) <= self.length():
			return False, self.length() - abs(self.x - i) - abs(self.y - j)
		return True, 0
	def length(self):
		return abs(self.x-self.bx) + abs(self.y - self.by)

sensors = []
se = set()
be = set()

def read():
	with open("INPUT") as f:
		minx = 9000000
		maxx = -9000000
		for line in f:
			t = line.split(' ')
			s = Sensor(int(t[2][2:-1]), int(t[3][2:-1]), int(t[8][2:-1]), int(t[9][2:-1]))
			sensors.append(s)
			minx = min(s.x, s.bx, minx)
			maxx = max(s.x, s.bx, maxx)
			se.add((s.x,s.y))
			be.add((s.bx, s.by))
		return minx, maxx

def compute(index, minx, maxx):
	addd = max([s.length() for s in sensors])
	minx -= addd
	maxx += addd
	m = [ False for _ in range(minx, maxx+1)]
	for s in sensors:
		s.map(m, minx, maxx, addd, index)
	summ = 0
	for i, c in enumerate(m):
		if(i + minx, index) in be or (i+minx, index) in se:
			continue
		if c:
			summ += 1
	print(f"Prvni: {summ}")

def compute2(fr, to):
	mset = set()
	for i in range(to - fr + 1):
		j = 0
		while j < (to - fr):
			found = True
			for s in sensors:
				f, shift = s.map2(i,j)
				found = found and f
				if not found:
					j += shift
					break
			j += 1
			if found:
				j -= 1
				print(f"Druhy: {i*4000000 + j}")
				return

minx, maxx = read()
compute(2000000, minx, maxx)
compute2(0, 4000000)
