#!/usr/bin/env python3

raw = list(map(list, open("day6_orig.txt").readlines()))
for i in range(len(raw[0])):
    if any(raw[j][i].isdigit() for j in range(4)):
        for j in range(4):
            if raw[j][i] == ' ':
                raw[j][i] = '0'

_ = open("day6.txt", "w").write("".join(map(lambda l: "".join(l), raw)))
