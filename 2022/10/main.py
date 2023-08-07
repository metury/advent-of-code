#!/usr/bin/env python3

round_count = 0
X = 1
total_sum = 0
CHECKING = [20,60,100,140,180,220]
drawing = [[False for _ in range(40)] for _ in range(7)]
sprite = [-1,0,1]

def step(rc, drw, spr, x, ts):
	mod = rc % 40
	div = rc // 40
	rc += 1
	drw[div][mod] = (mod in spr)
	if rc in CHECKING:
		ts += (rc * x)
	return rc, drw, spr, x, ts

with open("INPUT") as f:
	for line in f:
		if line == '\n':
			continue
		if line == 'noop\n':
			round_count, drawing, sprite, X, total_sum = step(round_count, drawing, sprite, X, total_sum)
		else:
			parts = line.split(" ")
			round_count, drawing, sprite, X, total_sum = step(round_count, drawing, sprite, X, total_sum)
			round_count, drawing, sprite, X, total_sum = step(round_count, drawing, sprite, X, total_sum)
			X += int(parts[1])
			sprite = [X-1, X, X+1]
			
print(f"Prvni: {total_sum}")
print(f"Druhy: [RZHFGJCB]")
for i in range(len(drawing)):
	for j in range(len(drawing[i])):
		if drawing[i][j]:
			print('#', end = "")
		else:
			print('.', end = "")
	print()
