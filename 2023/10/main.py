#!/usr/bin/env python3

from shapely import Polygon

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

def outline_and_coords(sewers, starting_point):
	""" Create an outline of the pipeline.
	    And also coordinates of the curved boundaries. """
	pipeline = [[False for _ in x] for x in sewers]
	coords = []
	
	pipeline[starting_point[0]][starting_point[1]] = True
	coords.append((starting_point[0],starting_point[1]))
	
	pos = find_start_pos(sewers, starting_point)
	
	pipeline[pos[1]][pos[2]] = True
	if sewers[pos[1]][pos[2]] != UP_DOWN and sewers[pos[1]][pos[2]] != LEFT_RIGHT:
		coords.append((pos[1],pos[2]))
	
	pos = step(pos[0], sewers, pos[1], pos[2])
	while sewers[pos[1]][pos[2]] != START:
		pipeline[pos[1]][pos[2]] = True
		if sewers[pos[1]][pos[2]] != UP_DOWN and sewers[pos[1]][pos[2]] != LEFT_RIGHT:
			coords.append((pos[1],pos[2]))
		pos = step(pos[0], sewers, pos[1], pos[2])
	return pipeline, coords

def distance(outline):
	""" Compute the distance of the pipeline. """
	my_sum = 0
	for x in outline:
		for y in x:
			if y:
				my_sum += 1
	return my_sum

with open('INPUT') as f:
	for line in f:
		sewer_line = [x for x in line.strip()]
		if START in sewer_line:
			starting_point = (len(sewers), sewer_line.index(START))
		sewers.append(sewer_line)

pipeline, coords = outline_and_coords(sewers, starting_point)
dist = distance(pipeline)
# Create a polygon from the coordiantes.
pgon = Polygon(coords)
# Use Pick's Theorem
# https://en.wikipedia.org/wiki/Pick%27s_theorem
interior = int(pgon.area +1 - dist / 2)
print(f"First part: { dist // 2}")
print(f"Second part: {interior}")
