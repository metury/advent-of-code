#!/usr/bin/env python3

with open("INPUT") as f:
    first = True
    plates = []
    second_plates = []
    rules = []
    for line in f:
        if first:
            first = False
            for j in range(len(line)-1):
                plates.append(line[j])
        else:
            t = line.split(" -> ")
            t[1] = t[1][0]
            temp = [t[0], t[1]]
            rules.append(temp)
    for k in range(10):
        #print(f"Step: {k}: {plates}")
        second_plates = []
        second_plates.append(plates[0])
        for i in range(1, len(plates)):
            frst = plates[i-1]
            scnd = plates[i]
            adding = ""
            find = frst + scnd
            for j in range(len(rules)):
                if rules[j][0] == find:
                    adding = rules[j][1]
                    break
            second_plates.append(adding)
            second_plates.append(scnd)
        plates = second_plates
    found = []
    for i in range(len(plates)):
        find = False
        for j in range(len(found)):
            if found[j][0] == plates[i]:
                found[j][1] += 1
                find = True
        if find == False:
            found.append([plates[i], 0])
    maxximum = ["", 0]
    minimum = ["", 0]
    for i in range(len(found)):
        if maxximum[0] == "" or maxximum[1] < found[i][1]:
            maxximum[0] = found[i][0]
            maxximum[1] = found[i][1]
        if minimum[0] == "" or minimum[1] > found[i][1]:
            minimum = found[i]
    print(maxximum[1] - minimum[1])
