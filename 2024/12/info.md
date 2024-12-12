#### Part 1

We will firstly use bucket fill algorithm to find the inner tiles and also the fences with their positions, orientations and sides. After that in the first part we multiple the number of fences with the number of fences.

#### Part 2

In the second part we compare the fences between each other if they are in the same side or not. We also abuse the fact, that we set UPPER and LOWER to even numbers and LOWER and RIGHT to odd numbers. Hence the horizontal ones have parity `0` and vertical `1`.
