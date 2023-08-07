#!/usr/bin/env python3

length = ord('z') - ord('a')

def index(char):
	if ord(char) > ord('Z'):
		return ord(char) - ord('a')
	else:
		return ord(char) - ord('A') + length + 1

total_sum = 0
type_sum = 0

with open("INPUT") as f:
	current = 0
	types = [True for _ in range(2*length + 2)]
	for line in f:
		if len(line) > 0:
			first_half = [0 for _ in range(2*length + 2)]
			second_half = [0 for _ in range(2*length + 2)]
			current_types = [False for _ in range(2*length + 2)]
			mid = (len(line) - 1)//2
			one = line[:mid]
			two = line[mid:]
			for i in range(mid):
				first_half[index(one[i])] += 1
				second_half[index(two[i])] += 1
				current_types[index(one[i])] = True
				current_types[index(two[i])] = True
			for j in range(2*length + 2):
				total_sum += min(min(first_half[j], second_half[j]),1) * (j+1)
				types[j] = types[j] and current_types[j]
			current += 1
			if current == 3:
				current = 0;
				for j in range(2*length + 2):
					if types[j]:
						type_sum += j+1
						break
				types = [True for _ in range(2*length + 2)]
						
			
print(f"Prvni: {total_sum}")
print(f"Druha: {type_sum}")
