#!/usr/bin/env python3

def differences(array1, array2):
	return sum([array1[i] != array2[i] for i in range(len(array1))])

def column_symmetry(matrix, smudges):
	for i in range(1, len(matrix[0])):
		shift = 0
		total = 0
		while i - shift - 1 >= 0 and i + shift <= len(matrix[0]) - 1 and total <= smudges:
			first_column = [matrix[j][i + shift] for j in range(len(matrix))]
			second_column = [matrix[j][i - shift - 1] for j in range(len(matrix))]
			total += differences(first_column, second_column)
			shift += 1
		if total == smudges:
			return i
	return 0

def row_symmetry(matrix, smudges):
	for i in range(1, len(matrix)):
		shift = 0
		total = 0
		while i - shift - 1 >= 0 and i + shift <= len(matrix) - 1 and total <= smudges:
			first_row = matrix[i + shift]
			second_row = matrix[i - shift - 1]
			total += differences(first_row, second_row)
			shift += 1
		if total == smudges:
			return i
	return 0

def compute(matrix, smudges):
	ret = 0
	if smudges:
		ret = column_symmetry(matrix, 1)
		ret += row_symmetry(matrix, 1) * 100
	else:
		ret = column_symmetry(matrix, 0)
		ret += row_symmetry(matrix, 0) * 100
	return ret

def find_mirror(smudges):
	with open('INPUT') as f:
		total = 0
		matrix = []
		for line in f:
			if line.strip() == "":
				total += compute(matrix, smudges)
				matrix = []
			else:
				matrix.append([x for x in line.strip()])
		total += compute(matrix, smudges)
		return total

print(f"First part: {find_mirror(False)}")
print(f"Second part: {find_mirror(True)}")
