#!/usr/bin/env python3

import operator

class Monkey:
	def __init__(self, items, operation, test, trmon, flsmon, nr):
		self.items = items
		self.operation = operation
		self.test = test
		self.counter = 0
		self.falsemonkey = flsmon
		self.truemonkey = trmon
		if nr == 'old':
			self.nr = 'old'
		else:
			self.nr = int(nr)
	def bind(self, true, false):
		self.true = true
		self.false = false
	def set_mod(self,mod):
		self.mod = mod
	def single_oper(self, item, div, mod):
		if self.nr == 'old':
			titem = self.operation(item,item)
		else:
			titem = self.operation(item, self.nr)
		if div:
			titem = titem // 3
		else:
			titem = titem % mod
		if titem % self.test == 0:
			self.true.add(titem)
		else:
			self.false.add(titem)
	def operate(self, mod, div):
		self.counter += len(self.items)
		for item in self.items:
			self.single_oper(item, div, mod)
		self.items = []
	def add(self, item):
		self.items.append(item)
	def print(self):
		print(self.items)


def read():
	monkeys = []
	with open("INPUT") as f:
		tmplist = []
		tmpoper = operator.mul;
		tmptest = 1
		truemonkey = 0
		falsemonkey = 0
		tmnr = 0
		tmmod = 1
		for line in f:
			parts = line.split(' ')
			if len(parts) < 2:
				monkeys.append(Monkey(tmplist, tmpoper, tmptest, truemonkey, falsemonkey, tmnr))
				tmplist = []
			elif parts[0] == 'Monkey':
				continue
			elif parts[3] == 'items:':
				for nr in parts[4:]:
					tmplist.append(int(nr[:-1]))
			elif parts[3] == 'new':
				if parts[6] == '+':
					tmpoper = operator.add;
				else:
					tmpoper = operator.mul;
				tmnr = parts[7][:-1]
			elif parts[3] == 'divisible':
				tmptest = int(parts[5])
				tmmod *= int(parts[5])
			elif parts[5] == 'true:':
				truemonkey = int(parts[9])
			elif parts[5] == 'false:':
				falsemonkey = int(parts[9])
	for monkey in monkeys:
		monkey.bind(monkeys[monkey.truemonkey], monkeys[monkey.falsemonkey])
	return monkeys, tmmod

def common(div, count):
	monkeys, mod = read()
	for i in range(count):
		for monkey in monkeys:
			monkey.operate(mod, div)
	max1 = 0
	max2 = 0
	for monkey in monkeys:
		if max1 < monkey.counter:
			max2 = max1
			max1 = monkey.counter
		elif max2 < monkey.counter:
			max2 = monkey.counter
	return max1*max2

def first_part():
	print(f"Prvni: {common(True, 20)}")
	
def second_part():
	print(f"Druhy: {common(False, 10000)}")

first_part()
second_part()
