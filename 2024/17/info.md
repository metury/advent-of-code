#### Part 1

In the first part we reconstruct the `computer` and simulate the given program.

#### Part 2

In the second part we may see that there is only one instruction which changes `A` that is `0 X` where `X <= 3`. So we construct the program from back. In each step we iterate over all numbers between `0` and `2^X`. If on the current position we have the same number as in the program itself we recourse on the previous index, also we multiply the current solution by `2^X` so that the tail stays the same. In other iterations we have some previous solution and add the new number for the current one. We also need to check if we don't have too long array of numbers and properly end.

