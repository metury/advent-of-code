#!/usr/bin/env python3

with open("INPUT") as f:
    line = f.readline()
    splitted = line.split(',')
    integers = []
    for box in splitted:
        integers.append(int(box))
    for i in range(80):
        for i in range(len(integers)):
            integers[i] -= 1
            if integers[i] == -1:
                integers[i] = 6
                integers.append(8)
    sum = len(integers)
    print(sum)
