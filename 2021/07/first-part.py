#!/usr/bin/env python3

with open("input.txt") as f:
    line = f.readline()
    separated = line.split(",")
    integers = []
    for box in separated:
        integers.append(int(box))
    integers.sort()
    middle = integers[len(integers)//2]
    print(middle)
    sum = 0
    for sub in integers:
        if sub <= middle:
            sum += (middle - sub)
        else:
            sum += (sub - middle)
    print(sum)
