#!/usr/bin/env python3

with open("INPUT") as f:
    array = []
    for i in range(1000):
        array.append([])
        for j in range(1000):
            array[i].append(0)
    for line in f:
        separate = line.split(" -> ")
        first = separate[0].split(",")
        second = separate[1].split(",")
        x1 = int(first[0])
        y1 = int(first[1])
        x2 = int(second[0])
        y2 = int(second[1])
        if x1 == x2:
            if y1 > y2:
                y1, y2 = y2, y1
            for i in range(y1, y2+1):
                array[x1][i] += 1
        elif y1 == y2:
            if x1 > x2:
                x1, x2 = x2, x1
            for i in range(x1, x2+1):
                array[i][y1] += 1
    sum = 0
    for line in array:
        for box in line:
            if box >= 2:
                sum += 1
    print(sum)
