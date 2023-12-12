#!/usr/bin/env python3

first_sum = 0
second_sum = 0

def count(array):
	counter = []
	current = 0
	for c in array:
		if c == "." and current > 0:
			counter.append(current)
			current = 0
		if c == "#":
			current += 1
	if current > 0:
		counter.append(current)
	#print(f"Ar: {array}, counter: {counter}")
	return counter

def compare(counter, image):
	if len(counter) != len(image):
		return False
	for i in range(len(counter)):
		if counter[i] != image[i]:
			return False
	return True

def try_fast(array, image, index, i_index, current):
	counter = [image[i] for i in range(i_index)]
	if i_index >= len(image):
		return 0
	while index < len(array) and array[index] != "?":
		if array[index] == "." and current > 0:
			if image[i_index] == current:
				i_index += 1
				counter.append(current)
				current = 0
			else:
				return 0
		if array[index] == "#":
			current += 1
		index += 1
	if i_index >= len(image) and counter == image:
		return True
	elif i_index >= len(image):
		return False
	if index >= len(array) or (index == len(array) - 1 and array[index] != "?"):
		if (i_index >= len(image) - 1 and counter == 0) or counter == image[i_index]:
			return 1
		else:
			return 0
	if current == image[i_index]:
		array[index] = "."
		return try_fast(array, image, index + 1, i_index, 0)
	if current == 0:
		ar1 = array.copy()
		ar1[index] = "."
		ar2 = array.copy()
		ar2[index] = "#"
		return try_fast(ar1, image, index + 1, i_index, 0) + try_fast(ar2, image, index + 1, i_index, 1)
	array[index] = "#"
	return try_fast(array, image, index + 1, i_index, current + 1)

def try_all(array, image, index):
	while index < len(array) and array[index] != "?":
		index += 1
	if index >= len(array) or (index == len(array) - 1 and array[index] != "?"):
		if compare(count(array), image):
			return 1
		return 0
	ar1 = array.copy()
	ar2 = array.copy()
	ar1[index] = "."
	ar2[index] = "#"
	return try_all(ar1, image, index + 1) + try_all(ar2, image, index + 1)

def multiply(array, image):
	origin = array.copy()
	origin_image = image.copy()
	for i in range(4):
		array.append("?")
		for c in origin:
			array.append(c)
		for i in origin_image:
			image.append(i)

with open('INPUT') as f:
	for line in f:
		parts = line.strip().split(" ")
		array = [x for x in parts[0]]
		image = [int(x) for x in parts[1].split(",")]
		print(image)
		print(array)
		print(try_all(array, image, 0))
		print(try_fast(array, image, 0, 0, 0))
		first_sum += try_fast(array, image, 0, 0, 0)
		multiply(array, image)
		#print(array)
		#print(image)
		#second_sum += try_all(array, image, 0)

print(f"First part: {first_sum}")
