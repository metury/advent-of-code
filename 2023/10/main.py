#!/usr/bin/env python3

""" ==Constants== """
START = 'S'
EMPTY = '.'
UP_DOWN = '|'
LEFT_RIGHT = '-'
DOWN_RIGHT = 'F'
DOWN_LEFT = '7'
UP_RIGHT = 'L'
UP_LEFT = 'J'
DOWN = 0
UP = 1
LEFT = 2
RIGHT = 3

""" ==Initialization== """
sewers = []
starting_point = (-1,-1)

def step(init_pos, sewers, x, y):
	""" Simulate one step from init_pos at (x,y)
	    in the sewers. """
	pipe = sewers[x][y]
	if pipe == UP_RIGHT:
		if init_pos == DOWN:
			return (RIGHT, x, y + 1)
		else:
			return (UP, x - 1, y)
	elif pipe == UP_LEFT:
		if init_pos == DOWN:
			return (LEFT, x, y - 1)
		else:
			return (UP, x - 1, y)
	elif pipe == UP_DOWN:
		if init_pos == UP:
			return (UP, x - 1, y)
		else:
			return (DOWN, x + 1, y)
	elif pipe == LEFT_RIGHT:
		if init_pos == LEFT:
			return (LEFT, x, y - 1)
		else:
			return (RIGHT, x, y + 1)
	elif pipe == DOWN_LEFT:
		if init_pos == UP:
			return (LEFT, x, y - 1)
		else:
			return (DOWN, x + 1, y)
	elif pipe == DOWN_RIGHT:
		if init_pos == UP:
			return (RIGHT, x, y + 1)
		else:
			return (DOWN, x + 1, y)

def find_start_pos(sewers, starting_point):
	""" Find one of the endpoints to the starting point. """
	x = starting_point[0]
	y = starting_point[1]
	if y + 1 < len(sewers[x]) and (sewers[x][y+1] == LEFT_RIGHT or sewers[x][y+1] == UP_LEFT or sewers[x][y+1] == DOWN_LEFT):
		return (RIGHT, x, y + 1)
	elif y - 1 >= 0 and (sewers[x][y-1] == LEFT_RIGHT or sewers[x][y-1] == UP_RIGHT or sewers[x][y+1] == DOWN_RIGHT):
		return (LEFT, x, y - 1)
	elif x + 1 < len(sewers) and (sewers[x+1][y] == UP_DOWN or sewers[x+1][y] == DOWN_RIGHT or sewers[x+1][y] == DOWN_LEFT):
		return (UP, x + 1, y)
	else:
		return (DOWN, x - 1, y)

def distance(sewers, starting_point):
	""" Compute the distance of the pipeline. """
	count = 1
	pos = find_start_pos(sewers, starting_point)
	pos = step(pos[0], sewers, pos[1], pos[2])
	while sewers[pos[1]][pos[2]] != START:
		pos = step(pos[0], sewers, pos[1], pos[2])
		count += 1
	return count + 1

def outline(sewers, starting_point):
	""" Create an outline of the pipeline. """
	pipeline = [[False for _ in x] for x in sewers]
	pipeline[starting_point[0]][starting_point[1]] = True
	pos = find_start_pos(sewers, starting_point)
	pipeline[pos[1]][pos[2]] = True
	pos = step(pos[0], sewers, pos[1], pos[2])
	while sewers[pos[1]][pos[2]] != START:
		pipeline[pos[1]][pos[2]] = True
		pos = step(pos[0], sewers, pos[1], pos[2])
	return pipeline

def updated_outlines(sewers, outline):
	outlines = [["." for _ in x] for x in outline]
	for i in range(len(sewers)):
		for j in range(len(sewers[i])):
			if sewers[i][j] == "|" and outline[i][j]:
				outlines[i][j] = "|"
			elif sewers[i][j] == "-" and outline[i][j]:
				outlines[i][j] = "-"
			elif outline[i][j]:
				outlines[i][j] = "#"
			print(outlines[i][j], end="")
		print()
	return outlines

def compute_inline(upd_out):
	up_down = [False for _ in upd_out[0]]
	my_sum = 0
	for i in range(len(upd_out)):
		left_right = False
		for j in range(len(upd_out[i])):
			if upd_out[i][j] == "#": # This is wrong.
				left_right = not left_right
				up_down[j] = not up_down[j]
				print("#", end="")
			elif upd_out[i][j] == "|":
				left_right = not left_right
				print("|", end="")
			elif upd_out[i][j] == "-":
				up_down[j] = not up_down[j]
				print("-", end="")
			else:
				if up_down[j] and left_right:
					my_sum += 1
					print("I", end="")
				else:
					print("O", end="")
		print()
	print(my_sum)

def print_outline(pipeline):
	for x in pipeline:
		for y in x:
			if y:
				print("#", end="")
			else:
				print(".", end="")
		print()

with open('INPUT') as f:
	for line in f:
		sewer_line = [x for x in line.strip()]
		if START in sewer_line:
			starting_point = (len(sewers), sewer_line.index(START))
		sewers.append(sewer_line)


print(f"First part: {distance(sewers, starting_point) // 2}")
out = outline(sewers, starting_point)
print_outline(out)
print()
updated_outlines(sewers, out)
print()
compute_inline(updated_outlines(sewers, out))
