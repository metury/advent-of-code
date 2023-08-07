#!/usr/bin/env python3

WIN = 6
TIE = 3

win_sum = 0
opt_sum = 0
MODULO = 4
same = ord("X") - ord("A")

def addType(prio, pl, mod):
	tmp = (prio + 1 + pl)
	if tmp >= mod:
		return tmp - mod + 1
	return tmp

with open("INPUT") as f:
	for line in f:
		fir = line[0]
		sec = line[2]
		if ord(sec) - ord(fir) == same:
			win_sum += TIE
		elif (fir == "A" and sec == "Y") or (fir == "B" and sec == "Z") or (fir == "C" and sec == "X"):
			win_sum += WIN
		win_sum += addType(ord(sec) - ord("X"), 0, MODULO)
		if sec == "Z":
			opt_sum += WIN
			opt_sum += addType(ord(fir) - ord("A"), 1, MODULO)
		elif sec == "Y":
			opt_sum += TIE
			opt_sum += addType(ord(fir) - ord("A"), 0, MODULO)
		else:
			opt_sum += addType(ord(fir) - ord("A"), 2, MODULO)
			
print(f"První: {win_sum}")
print(f"Druhý: {opt_sum}")
