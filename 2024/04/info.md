#### Part 1

In the first part we create way bigger text by adding its transposition (for vertical ones) and also we add the main and second diagonal (by swapping the order of lines and diagonalization). Lastly we replace all `S` by `SS` and `X` by `XX` for the overlaping cases. Now for this text we run regex `XMAS|SAMX` and count the occurances.

#### Part 2

In the second part we used different approach. That is we iterate over the matrix and if we find `A` in the middle we create diagonal words and check if they are `SAM|MAS`.
