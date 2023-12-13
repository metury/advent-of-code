#!/usr/bin/env python3

def differences(array1, array2):
	return sum([array1[i] != array2[i] for i in range(len(array1))])

def column_symmetry(matrix):
	for i in range(1, len(matrix[0])):
		shift = 0
		while [matrix[j][i + shift] for j in range(len(matrix))] == [matrix[j][i - shift - 1] for j in range(len(matrix))]:
			if i - shift - 1 > 0 and i + shift < len(matrix[0]) - 1:
				shift += 1
			else:
				return i
	return 0

def smudge_column_symmetry(matrix):
	for i in range(1, len(matrix[0])):
		shift = 0
		total = 0
		while i - shift - 1 >= 0 and i + shift <= len(matrix[0]) - 1:
			total += differences([matrix[j][i + shift] for j in range(len(matrix))], [matrix[j][i - shift - 1] for j in range(len(matrix))])
			shift += 1
		if total == 1:
			return i
	return 0

def row_symmetry(matrix):
	for i in range(1, len(matrix)):
		shift = 0
		while matrix[i + shift] == matrix[i - shift - 1]:
			if i - shift - 1 > 0 and i + shift < len(matrix) - 1:
				shift += 1
			else:
				return i
	return 0

def smudge_row_symmetry(matrix):
	for i in range(1, len(matrix)):
		shift = 0
		total = 0
		while i - shift - 1 >= 0 and i + shift <= len(matrix) - 1:
			total += differences(matrix[i + shift], matrix[i - shift - 1])
			shift += 1
		if total == 1:
			return i
	return 0

def compute(matrix, smudges):
	ret = 0
	if smudges:
		ret = smudge_column_symmetry(matrix)
		ret += smudge_row_symmetry(matrix) * 100
	else:
		ret = column_symmetry(matrix)
		ret += row_symmetry(matrix) * 100
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
