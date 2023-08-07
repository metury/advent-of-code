#!/usr/bin/env python3

def read():
	forest = []
	with open("INPUT") as f:
		for line in f:
			tmp = line[:-1]
			forest.append([int(c) for c in tmp])
	return forest

def compute_visible(forest, visible, consti, constj, multi, multj, swap):
	for i in range(len(forest)):
		minimum = -1
		for j in range(len(forest[i])):
			if swap:
				i,j = j,i
			#print(f"Hledáme: {consti + multi*i} a {constj + multj*j}")
			if forest[consti + multi*i][constj + multj*j] > minimum:
				minimum = forest[consti + multi*i][constj + multj*j]
				visible[consti + multi*i][constj + multj*j] = True
			if swap:
				i,j = j,i
	return visible

def total_sum(visible):
	tot_sum = 0
	for line in visible:
		for cell in line:
			if cell:
				tot_sum += 1
	print(f"Prvni: {tot_sum}")

def first_part(forest):
	visible = [[False for _ in range(len(a))] for a in forest]
	visible = compute_visible(forest, visible, 0, 0, 1, 1, False)
	visible = compute_visible(forest, visible, 0, len(forest)-1, 1, -1, False)
	visible = compute_visible(forest, visible, 0, 0, 1, 1, True)
	visible = compute_visible(forest, visible, len(forest)-1, 0, -1,  1, True)
	total_sum(visible)

def around(i, j, length, multi, multj):
	current_view = 0
	for l in range(length):
		if forest[i+multi*(l+1)][j+multj*(l+1)] < forest[i][j]:
			current_view += 1
		else:
			current_view += 1
			break
	return current_view

def second_part(forest):
	max_score = 0
	for i in range(len(forest)):
		for j in range(len(forest[i])):
			score = 1
			score *= around(i,j, j, 0, -1)
			score *= around(i,j, len(forest[i]) - j - 1, 0, 1)
			score *= around(i,j, i, -1, 0)
			score *= around(i,j, len(forest) - i - 1, 1, 0)
			if score > max_score:
				max_score = score
	print(f"Druhy: {max_score}")

forest = read()
first_part(forest)
second_part(forest)
