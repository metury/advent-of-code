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

def compute_r_inline(sewers, outline):
	inbounds = [[False for _ in x] for x in outline]
	for i in range(len(sewers)):
		inbound = False
		continuous = 0
		for j in range(len(sewers[i])):
			if outline[i][len(sewers[i]) - 1 - j]:
				inbound = not inbound
				continuous += 1
			else:
				inbounds[i][len(sewers[i]) - 1 - j] = inbound or (continuous % 2 == 0 and continuous > 0)
				continuous = 0
			if inbounds[i][len(sewers[i]) - 1 - j]:
				print("I", end="")
			else:
				print(".", end="")
		print()
	print()
	return inbounds

def compute_f_inline(sewers, outline):
	inbounds = [[False for _ in x] for x in outline]
	for i in range(len(sewers)):
		inbound = False
		continuous = 0
		for j in range(len(sewers[i])):
			if outline[i][j]:
				inbound = not inbound
			else:
				inbounds[i][j] = inbound or (continuous % 2 == 0 and continuous > 0)
				continuous = 0
			if inbounds[i][j]:
				print("I", end="")
			else:
				print(".", end="")
		print()
	print()
	return inbounds

def alt_compute_inline(sewers, outline):
	f = compute_f_inline(sewers, outline)
	r = compute_r_inline(sewers, outline)
	inbounds = [[False for _ in x] for x in outline]
	for i in range(len(sewers)):
		for j in range(len(sewers[i])):
			inbounds[i][j] = f[i][j] and r[i][j]
			if inbounds[i][j]:
				print("I", end="")
			else:
				print(".", end="")
		print()
	return inbounds 

def compute_inline(sewers, outline):
	my_sum = 0
	for i in range(len(sewers)):
		inbound = False
		for j in range(len(sewers[i])):
			if outline[i][j]:
				inbound = not inbound
				if inbound:
					print("(", end = "")
				else:
					print(")", end = "")
			elif inbound and outline[i][j] != LEFT_RIGHT:
				my_sum += 1
				print("I", end="")
			else:
				print(sewers[i][j], end="")
		print()
	return my_sum

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
alt_compute_inline(sewers, out)
