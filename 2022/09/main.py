#!/usr/bin/env python3

class rope:
	def __init__(self, count):
		self.head = (0,0)
		self.nodes = [(0,0) for _ in range(count)]
		self.tail = (0,0)
		self.visited = {self.tail}
	def one_close(self, fir, sec, thi, fou, eq):
		if eq*(fir - sec) > eq**2:
			if thi == fou:
				second = thi
			elif thi > fou:
				second = thi - 1
			else:
				second = thi + 1
			first = fir - eq
			return first, second, True
		return fir, sec, False
	def is_close(self, node1, node2):
		first, second, found = self.one_close(node1[0], node2[0], node1[1], node2[1], 1)
		if found:
			return (first,second)
		first, second, found = self.one_close(node1[0], node2[0], node1[1], node2[1], -1)
		if found:
			return (first,second)
		second, first, found = self.one_close(node1[1], node2[1], node1[0], node2[0], 1)
		if found:
			return (first,second)
		second, first, found = self.one_close(node1[1], node2[1], node1[0], node2[0], -1)
		if found:
			return (first,second)
		return (node1[0], node1[1])
	def move(self, move, length):
		for _ in range(int(length)):
			if move == 'R':
				self.head = (self.head[0], self.head[1]-1)
			elif move == 'L':
				self.head = (self.head[0], self.head[1]+1)
			elif move == 'D':
				self.head = (self.head[0]-1, self.head[1])
			elif move == 'U':
				self.head = (self.head[0]+1, self.head[1])
			self.nodes_moves()
	def nodes_moves(self):
		tmp = self.head
		for i in range(len(self.nodes)):
			self.nodes[i] = self.is_close(self.nodes[i], tmp)
			tmp = self.nodes[i]
		self.tail = self.is_close(self.tail, tmp)
		self.visited.add(self.tail)
	def get_visited(self):
		return len(self.visited)

with open("INPUT") as f:
	my_rope = rope(0)
	sec_part = rope(8)
	for line in f:
		if line != '\n':
			tmp = line.split(' ')
			my_rope.move(tmp[0],tmp[1])
			sec_part.move(tmp[0],tmp[1])
	print(f"Prvni: {my_rope.get_visited()}")
	print(f"Druhy: {sec_part.get_visited()}")
