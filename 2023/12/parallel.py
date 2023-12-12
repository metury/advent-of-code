#!/usr/bin/env python3

from multiprocessing import Pool

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
	return counter

def compare(counter, image):
	if len(counter) != len(image):
		return False
	for i in range(len(counter)):
		if counter[i] != image[i]:
			return False
	return True

def try_fast(array, image, index, imdex, counter):
	#print(f"Array: {array}, image: {image}, index: {index}, imdex: {imdex}, counter: {counter}")
	if index >= len(array) and imdex >= len(image) and counter == 0:
		#print("True")
		return 1
	if index >= len(array) and imdex == len(image) - 1 and image[imdex] == counter:
		#print("True")
		return 1
	if index >= len(array):
		#print("False")
		return 0
	if array[index] == ".":
		if counter > 0:
			if imdex >= len(image) or counter != image[imdex]:
				#print("False")
				return 0
			if counter == image[imdex]:
				return try_fast(array, image, index + 1, imdex + 1, 0)
		return try_fast(array, image, index + 1, imdex, 0)
	if array[index] == "#":
		if imdex >= len(image) or counter + 1 > image[imdex]:
			#print("False")
			return 0
		return try_fast(array, image, index + 1, imdex, counter + 1)
	ar1 = array.copy()
	ar1[index] = "."
	ar2 = array.copy()
	ar2[index] = "#"
	if counter == 0:
		return try_fast(ar1, image, index + 1, imdex, counter) + try_fast(ar2, image, index + 1, imdex, counter + 1)
	if imdex < len(image) and counter == image[imdex]:
		return try_fast(ar1, image, index + 1, imdex + 1, 0)
	if imdex < len(image) and counter < image[imdex]:
		return try_fast(ar2, image, index + 1, imdex, counter + 1)
	#print("False")
	return 0

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

lines = []

print("[",end="")

with open('INPUT') as f:
	for line in f:
		lines.append(line)

def f(line):
	parts = line.strip().split(" ")
	array = [x for x in parts[0]]
	image = [int(x) for x in parts[1].split(",")]
	first = try_fast(array, image, 0, 0, 0)
	multiply(array, image)
	second = try_fast(array, image, 0, 0, 0)
	print("-",end="")
	return (first, second)

res = []

with Pool(7) as p:
	res = p.map(f, lines)

for c in res:
	first_sum += c[0]
	second_sum += c[1]

print("]")

print(f"First part: {first_sum}")
print(f"Second part: {second_sum}")
