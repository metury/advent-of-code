#!/usr/bin/env python3

seeds = []
range_seeds = []
"""
[(begining, size, True/False), ...
Each time we split one tuple.
"""

def interval(f, f_beg, t_beg, size):
	if f >= f_beg and f <= f_beg + size:
		return ((f - f_beg) + t_beg, True)
	return (0, False)

def range_interval(f, f_size, f_beg, t_beg, size):
	beg_inside = f >= f_beg and f <= f_beg + size
	end_inside = f + f_size >= f_beg and f + f_size <= f_beg + size
	outside = f < f_beg and f + f_size > f_beg + size

	interval_beg = f
	interval_end = f + f_size
	from_begining = f_beg
	from_end = f_beg + size
	shift = - f_beg + t_beg

	if beg_inside and end_inside:
		return [((f - f_beg) + t_beg, f_size, True)]
	elif beg_inside:
		return [(interval_beg + shift, from_end - interval_beg + 1, True), (from_end + 1, interval_end - from_end - 1, False)]
	elif end_inside:
		return [(from_begining + shift, interval_end - from_begining, True), (interval_beg, from_begining - interval_beg, False)]
	elif outside:
		return [(from_begining + shift, size, True), (interval_beg, from_begining - interval_beg + 1, False), (from_end, interval_end - from_end + 1, False)]
	else:
		return []

def reset(seeds):
	for i in range(len(seeds)):
		seeds[i] = (seeds[i][0], False)

def reset_range(seeds):
	for i in range(len(seeds)):
		seeds[i] = (seeds[i][0], seeds[i][1], False)

with open('INPUT') as f:
	for line in f:
		parts = line.strip().split(" ")
		if parts[0] == "seeds:":
			for i in range(1, len(parts)):
				seeds.append((int(parts[i]), False))
			for i in range(1, len(parts), 2):
				range_seeds.append((int(parts[i]), int(parts[i+1]), False))
		elif len(parts[0]) == 0 or not parts[0][0].isdigit():
			reset(seeds)
			reset_range(range_seeds)
		else:
			t_beg = int(parts[0])
			f_beg = int(parts[1])
			size = int(parts[2])
			for i in range(len(seeds)):
				if not seeds[i][1]:
					ret = interval(seeds[i][0], f_beg, t_beg, size)
					if ret[1]:
						seeds[i] = ret
			for i in range(len(range_seeds)):
				if not range_seeds[i][2]:
					ret = range_interval(range_seeds[i][0], range_seeds[i][1], f_beg, t_beg, size)
					if ret != []:
						range_seeds.pop(i)
					for j in range(len(ret)):
						range_seeds.insert(i, ret[j])

real_seeds = [x[0] for x in seeds]
real_range_seeds = [x[0] for x in range_seeds]
print(f"First part: {min(real_seeds)}")	
print(f"Second part: {min(real_range_seeds)}")
