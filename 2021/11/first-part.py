#!/usr/bin/env python3

with open("INPUT") as f:
    array = []
    for i in range(10):
        array.append([])
        for j in range(10):
            array[i].append(0)
    index = 0
    for line in f:
        j = 0
        for char in line:
            if char != '\n':
                array[index][j] = (int(char))
                j += 1
        index += 1
    sum = 0
    for i in range(100):
        for j in range(len(array)):
            for k in range(len(array[j])):
                array[j][k] += 1
        stop = 1
        while stop == 1:
            stop = 0
            for j in range(len(array)):
                for k in range(len(array[j])):
                    if array[j][k] > 9:
                        stop = 1
                        sum += 1
                        array[j][k] = 0
                        adjecent = [[j,k+1],[j+1,k],[j-1,k],[j,k-1],[j+1,k-1],[j-1,k+1],[j+1,k+1],[j-1,k-1]]
                        for ad in adjecent:
                            if ad[0] >= 0 and ad[1] >= 0 and ad[0] < len(array) and ad[1] < len(array[j]):
                                if (array[ad[0]][ad[1]] != 0):
                                    array[ad[0]][ad[1]] += 1
    print(sum)
