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

def distance(sewers, starting_point):
	""" Compute the distance of the pipeline. """
	out = outline(sewers, starting_point)
	my_sum = 0
	for x in out:
		for y in x:
			if y:
				my_sum += 1
	return my_sum

def updated_outlines(sewers, outline):
	""" Show pipes only in the loop. """
	outlines = [["." for _ in x] for x in outline]
	for i in range(len(sewers)):
		for j in range(len(sewers[i])):
			if outline[i][j]:
				outlines[i][j] = sewers[i][j]
	return outlines

def compute_inline(upd_out):
	""" This actually don't work for test input,
	    but it worked for my real data. :P"""
	my_sum = 0
	for i in range(len(upd_out)):
		lr = False
		for j in range(len(upd_out[i])):
			if upd_out[i][j] == UP_DOWN or upd_out[i][j] == DOWN_RIGHT or upd_out[i][j] == DOWN_LEFT:
				lr = not lr
			elif upd_out[i][j] == "." and lr:
				my_sum += 1
	return my_sum

with open('INPUT') as f:
	for line in f:
		sewer_line = [x for x in line.strip()]
		if START in sewer_line:
			starting_point = (len(sewers), sewer_line.index(START))
		sewers.append(sewer_line)


print(f"First part: {distance(sewers, starting_point) // 2}")
out = outline(sewers, starting_point)
upd = updated_outlines(sewers, out)
print(f"Second part: {compute_inline(upd)}")
