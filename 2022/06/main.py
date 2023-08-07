#!/usr/bin/env python3

def check(array):
	sarray = sorted(array)
	for i in range(len(array) - 1):
		if sarray[i] == sarray[i+1] or sarray[i] == -1:
			return False
	return True

with open("INPUT") as f:
	for line in f:
		four = [-1 for _ in range(4)]
		message = [-1 for _ in range(14)]
		current = 0
		current_msg = 0
		first = False
		for i,char in enumerate(line):
			four[current] = ord(char)
			message[current_msg] = ord(char)
			current = (current + 1) % 4
			current_msg = (current_msg + 1) % 14
			if check(four) and not first:
				print(f"První: {i+1}")
				first = True
			if check(message):
				print(f"Druhá: {i+1}")
				break
