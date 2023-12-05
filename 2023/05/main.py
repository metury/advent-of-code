#!/usr/bin/env python3

seeds = []
range_seeds = []

def interval(f, f_beg, t_beg, size):
	if f >= f_beg and f <= f_beg + size:
		return ((f - f_beg) + t_beg, True)
	return (0, False)

def range_interval(f, f_size, f_beg, t_beg, size):
	beg_inside = f >= f_beg and f <= f_beg + size
	end_inside = f + f_size >= f_beg and f + f_size <= f_beg + size:
	if beg_inside and end_inside:
		return [((f - f_beg) + t_beg, f_size, True)]
	elif beg_inside:
		return [((f - f_beg) + t_beg, f_size - size, True), ((t_beg + size + 1)

def reset(seeds):
	for i in range(len(seeds)):
		seeds[i] = (seeds[i][0], False)

with open('INPUT') as f:
	for line in f:
		parts = line.strip().split(" ")
		if parts[0] == "seeds:":
			for i in range(1, len(parts)):
				seeds.append((int(parts[i]), False))
		elif len(parts[0]) == 0 or not parts[0][0].isdigit():
			reset(seeds)
		else:
			t_beg = int(parts[0])
			f_beg = int(parts[1])
			size = int(parts[2])
			for i in range(len(seeds)):
				if not seeds[i][1]:
					ret = interval(seeds[i][0], f_beg, t_beg, size)
					if ret[1]:
						seeds[i] = ret

real_seeds = [x[0] for x in seeds]
print(f"First part: {min(real_seeds)}")	
