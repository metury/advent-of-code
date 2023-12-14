#!/usr/bin/env python3

matrix = []
INVALID = -1

def shift_column(column):
	free = INVALID
	for i in range(len(column)):
		if column[i] == '.' and free == INVALID:
			free = i
		elif column[i] == "#":
			free = INVALID
		elif column[i] == "O" and free != INVALID:
			column[free] = "O"
			column[i] = "."
			free += 1
	return column

def shift_all(matrix):
	for j in range(len(matrix[0])):
		column = [matrix[i][j] for i in range(len(matrix))]
		shifted = shift_column(column)
		for i in range(len(matrix)):
			matrix[i][j] = shifted[i]

def shift(matrix, N = False, W = False, S = False, E = False):
	if N:
		for j in range(len(matrix[0])):
			column = [matrix[i][j] for i in range(len(matrix))]
			shifted = shift_column(column)
			for i in range(len(matrix)):
				matrix[i][j] = shifted[i]
	elif S:
		for j in range(len(matrix[0])):
			column = [matrix[i][j] for i in range(len(matrix))]
			column.reverse()
			shifted = shift_column(column)
			shifted.reverse()
			for i in range(len(matrix)):
				matrix[i][j] = shifted[i]
	elif E:
		for j in range(len(matrix)):
			row = matrix[j].copy()
			shifted = shift_column(row)
			matrix[j] = shifted
	elif W:
		for j in range(len(matrix)):
			row = matrix[j].copy()
			row.reverse()
			shifted = shift_column(row)
			shifted.reverse()
			matrix[j] = shifted

def shift_cycle(matrix):
	shift(matrix, N = True)
	shift(matrix, W = True)
	shift(matrix, S = True)
	shift(matrix, E = True)

def compute(shifted_matrix):
	ret = 0
	for i in range(len(matrix)):
		ovals = sum([x == 'O' for x in matrix[i]])
		ret += ovals * (len(matrix) - i)
	return ret

with open('INPUT') as f:
	for line in f:
		matrix_line = [x for x in line.strip()]
		matrix.append(matrix_line)

# First part
#shift_all(matrix)
#print(compute(matrix))
last_one = matrix.copy()

for i in range(1_000_000_000):
	print(i)
	shift_cycle(matrix)
	if matrix == last_one:
		break
	last_one = matrix.copy()

shift(matrix, N = True)
print(compute(matrix))
