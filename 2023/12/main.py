#!/usr/bin/env python3

def count_placements(groups, clusters, cache = dict()):
	key = "|".join(map(str, groups))
	key += "#" + ":".join(clusters)
	if key in cache:
		return cache[key]

	if not groups:
		for cluster in clusters:
			if '#' in cluster:
				return 0
		return 1
	if not clusters:
		return 0
	ret = 0
	cluster = clusters[0]
	group = groups[0]
	len_cluster = len(cluster)
	if group > len_cluster and '#' in cluster:
		return 0
	for i in range(len_cluster - group + 1):
		left = cluster[:i]
		if '#' in left:
			continue
		right = cluster[i + group:]
		if right.startswith('#'):
			continue
		new_clusters = clusters[1:]
		if len(right) > 1:
			new_clusters.insert(0, right[1:])
		ret += count_placements(groups[1:], new_clusters, cache)

	if '#' not in cluster:
		ret += count_placements(groups, clusters[1:], cache)

	cache[key] = ret

	return ret


def compute(expand):
	total = 0
	with open('INPUT', 'r') as file:
		for line in file:
			springs, groups_str = line.strip().split()
			groups = [int(x) for x in groups_str.split(",")]
			if expand:
				springs = "?".join(5 * [springs])
				groups = 5 * groups
			clusters = [s for s in springs.split('.') if s]
			delta = count_placements(groups, clusters)
			total += delta
	return total

def part1():
	return compute(False)

def part2():
	return compute(True)

print(f"First part: {part1()}")
print(f"Second part: {part2()}")
