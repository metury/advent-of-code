#!/usr/bin/env python3

calories_max = 0
calories_sec_max = 0
calories_third_max = 0

with open('INPUT') as f:
	buffer = 0
	for line in f:
		if line == "\n":
			if buffer > calories_max:
				calories_third_max = calories_sec_max
				calories_sec_max = calories_max
				calories_max = buffer
			elif buffer > calories_sec_max:
				calories_third_max = calories_sec_max
				sec_max = buffer
			elif buffer > calories_third_max:
				calories_third_max = buffer
			buffer = 0			
		else:
			buffer += int(line)

print(f"První: {calories_max}")
print(f"Druhý: {calories_max + sec_max + third_max}")
