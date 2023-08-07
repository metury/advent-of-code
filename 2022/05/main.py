#!/usr/bin/env python3

def frearrange(stack, count, fstack, tostack):
	for _ in range(count):
		if (stack[fstack] != []):
			stack[tostack].append(stack[fstack][-1])
			stack[fstack] = stack[fstack][:-1]

def srearrange(stack, count, fstack, tostack):
	realCount = min(count, len(stack[fstack]))
	tmp = stack[fstack][-realCount:]
	stack[fstack] = stack[fstack][:-realCount]
	for i in range(len(tmp)):
		stack[tostack].append(tmp[i])

def compute(rearrange, text):
	stack = [[] for _ in range(9)]
	with open("INPUT") as f:
		for line in f:
			if line != '\n':
				if line[0] != 'm' and line[1] != '1':
					for i in range(len(stack)):
						if(line[(4*i)+1]) != ' ':
							stack[i].append(line[(4*i)+1])
				elif line[1] == '1':
					for i in range(len(stack)):
						stack[i] = stack[i][::-1]
				elif line[0] == 'm':
					comp = line.split(' ')
					count = int(comp[1])
					fstack = int(comp[3]) - 1
					tostack = int(comp[5]) - 1
					rearrange(stack, count, fstack, tostack)
	print(text, end = "")
	for i in range(len(stack)):
		if stack[i] != []:
			print(stack[i][-1], end = "")
	print()

compute(frearrange, "Prvni: ")
compute(srearrange, "Druhy: ")
