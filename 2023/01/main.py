#!/usr/bin/env python3
import ahocorasick

# Initialize everythink
first_sum = 0
second_sum = 0
numbers = ['one', 'two', 'three', 'four', 'five', 'six', 'seven', 'eight', 'nine']
automaton = ahocorasick.Automaton()
for idx, key in enumerate(numbers):
	automaton.add_word(key, (idx, key))

def get_real_numbers(line):
	""" Get real numbers in string.
		The first and last appearance.
		-1 is for invalid.             """
	first = -1
	first_index = -1
	last = -1
	last_index = -1
	for i, ch in enumerate(line):
		if ord(ch) >= ord('0') and ord(ch) <= ord('9'):
			last = int(ch)
			last_index = i
			if first == -1:
				first = int(ch)
				first_index = i
	return first, last, first_index, last_index

def my_min(array):
	""" Find the min except for -1. """
	val = max(array)
	for a in array:
		if a != -1 and a < val:
			val = a
	return val
	
def find(line):
	""" Find the first and last occurance of text numbers.
	    Return array of tuples.                            """
	appearances = [(-1,-1) for _ in numbers]
	automaton.make_automaton()
	for end_index, (index, original_value) in automaton.iter(line):
		start_index = end_index - len(original_value) + 1
		appearances[index] = (my_min([appearances[index][0], start_index]), max(appearances[index][1], start_index))
	return appearances

def update(app, f, fi, l, li):
	""" Update the array so there are also the real numbers. """
	app[f - 1] = (my_min([app[f-1][0], fi]), max(app[f-1][1], fi))
	app[l - 1] = (my_min([app[l-1][0], li]), max(app[l-1][1], li))

with open('INPUT') as f:
	for line in f:
		appearances = find(line)
		first, last, first_index, last_index = get_real_numbers(line)
		update(appearances, first, first_index, last, last_index)
		first_sum += first * 10 + last
		mins = [x[0] for x in appearances]
		maxs = [x[1] for x in appearances]
		second_sum += (mins.index(my_min(mins)) + 1)*10 + maxs.index(max(maxs)) + 1

print(f"First part: {first_sum}")
print(f"Second part: {second_sum}")
