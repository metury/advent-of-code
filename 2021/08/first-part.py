#!/usr/bin/env python3

with open("INPUT") as f:
    stack = []
    sum = 0
    for line in f:
        for char in line:
            if char == "<" or char == "(" or char == "{" or char == "[":
                stack.append(char)
            elif char == ">" or char == ")" or char == "}" or char == "]":
                temp = stack.pop()
                if (temp == "<" and char != ">") or (temp == "(" and char != ")") or (temp == "[" and char != "]") or (temp == "{" and char != "}"):
                    if char == ">":
                        sum += 25137
                    elif char == "}":
                        sum += 1197
                    elif char == ")":
                        sum += 3
                    elif char == "]":
                        sum += 57
    print(sum)
