#### Part 1

In the first part we only match the regex `mul\(([0-9]{1,3}),([0-9]{1,3})\)` and compute the result.

#### Part 2

In the second part we also add `do()|don't()` and iterate over found values. If we find `do()` enable computing otherwise for `don't()` disable it. Then compute the result.
