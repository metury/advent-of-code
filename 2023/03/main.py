#!/usr/bin/env python3

triplets = [[] for _ in range(3)]
first_sum = 0
second_sum = 0

GEAR = "*"
TAIL = -1

def is_special(char):
	return char != '.' and not char.isdigit()

def transform_row(row):
	""" Get the numbers and put it in the begining position
	    and as the tail insert -1. Otherwise 0. """
	number = 0
	index = 0
	new_row = [0 for _ in row]
	for i in range(len(row)):
		if row[i].isdigit():
			if number == 0:
				index = i
			number = number * 10 + int(row[i])
		else:
			new_row[index] = number
			for j in range(index+1, i):
				new_row[j] = TAIL
			index = i
			number = 0
	return new_row

def get_number(row, index, r_index):
	""" Get the number at this index in the row. """
	if index < 0 or index >= len(row):
		return (0, -1)
	i = 0
	while row[index - i] == TAIL:
		i += 1
	return (row[index - i], index - i, r_index)

def scan_gears(t_triplets, triplets):
	""" Scan the gears for the middle triplet. """
	my_sum = 0
	for i in range(len(triplets[1])):
		if triplets[1][i] == GEAR:
			numbers = [get_number(t_triplets[0], i-1, 0),
			           get_number(t_triplets[0], i  , 0),
			           get_number(t_triplets[0], i+1, 0),
			           get_number(t_triplets[1], i-1, 1),
			           get_number(t_triplets[1], i+1, 1),
			           get_number(t_triplets[2], i-1, 2),
			           get_number(t_triplets[2], i  , 2),
			           get_number(t_triplets[2], i+1, 2)]
			numbers = filter(lambda part : part[0] > 0, numbers)
			numbers = list(dict.fromkeys(numbers))
			if len(numbers) == 2:
				my_sum = my_sum + numbers[0][0] * numbers[1][0]
	return my_sum

def sum_middle(row, scan):
	""" Sum those numbers that are scanned.
	    At least one scan throughout the number. """
	number = 0
	my_sum = 0
	scanned = False
	for i in range(len(row)):
		if row[i].isdigit():
			number = number * 10 + int(row[i])
			if scan[i]:
				scanned = scan[i]
		else:
			if scanned:
				my_sum = my_sum + number
			number = 0
			scanned = False
	if scanned and number != 0:
		my_sum = my_sum + number
	return my_sum

def scan_middle(triplets):
	""" Scan the middle row if each char has
	    special character near him. """
	scanning = [False for _ in triplets[1]]
	for i in range(len(triplets[1])):
		if i > 0 and is_special(triplets[1][i-1]):
			scanning[i] = True
		elif i < len(triplets[1]) - 1 and is_special(triplets[1][i+1]):
			scanning[i] = True
		for j in range(i - 1, i+2):
			for t in [triplets[0], triplets[2]]:
				if j >= 0 and j < len(t) and is_special(t[j]):
					scanning[i] = True
	return sum_middle(triplets[1], scanning)

with open('INPUT') as f:
	for line in f:
		triplets[0], triplets[1] = triplets[1], triplets[2]
		triplets[2] = [c for c in line.strip()]
		first_sum += scan_middle(triplets)
		second_sum += scan_gears([transform_row(triplets[0]), transform_row(triplets[1]), transform_row(triplets[2])], triplets)
	triplets[0], triplets[1], triplets[2] = triplets[1], triplets[2], []
	first_sum += scan_middle(triplets)
	second_sum += scan_gears([transform_row(triplets[0]), transform_row(triplets[1]), transform_row(triplets[2])], triplets)

print(f"First part: {first_sum}")
print(f"Second sum: {second_sum}")
