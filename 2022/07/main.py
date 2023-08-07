#!/usr/bin/env python3

class directory:
	def __init__(self, name, parent = ""):
		self.name = name
		self.parent = parent
		self.subdirs = []
		self.files = []
		self.tot_sum = 0
	def sum_all(self):
		for f in self.files:
			self.tot_sum += f
		for d in self.subdirs:
			self.tot_sum += d.sum_all()
		return self.tot_sum
	def add_all(self):
		tmp = 0
		if self.tot_sum <= 100000:
			tmp += self.tot_sum
		for d in self.subdirs:
			tmp += d.add_all()
		return tmp
	def return_min(self, size):
		if self.tot_sum > size:
			tmp = self.tot_sum
		else:	
			tmp = 70000000
		for d in self.subdirs:
			tmp = min(tmp, d.return_min(size))
		return tmp
	def same_name(self, name):
		return name == self.name
	def add_subdir(self, n, c):
		self.subdirs.append(directory(n, c))
	def cd(self, name):
		if name == '..\n':
			return self.parent
		for d in self.subdirs:
			if d.same_name(name):
				return d

root = directory('/')

with open("INPUT") as f:
	current = root
	for line in f:
		if line != '\n':
			if line[0] == '$':
				if line[2] == 'c':
					tmp = line.split(' ')
					if tmp[2] == '/\n':
						current = root
					else:
						current = current.cd(tmp[2])
			elif line[0] == 'd':
				current.add_subdir((line.split(' '))[1], current)
			else:
				number = int((line.split(' '))[0])
				current.files.append(number)

space = 70000000 - root.sum_all() 
needed = 30000000 - space
print(f"Prvni {root.add_all()}")
print(f"Druhy {root.return_min(needed)}")
