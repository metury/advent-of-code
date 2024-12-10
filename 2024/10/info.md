#### Part 1

In the first part we use recursion on the nighbours. But beforehand we check if the value is `9`, if so add it to the `map`. We also check if neighbours are indeed inside of the map and if the value on this place is `1` higher than the current one.

#### Part 2

In this case we do not need `map` and just return on if `9` was found, and recourse on counting such cases. That is we get to the nine number as many times as there are distinct paths.

