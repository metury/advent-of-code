#### Part 1

For both cases we go through the map and collect the positions of each antenna type. Then in the first part we go through the antennas of the same type and take two of them. Calculating their difference and later on using this difference to either `subtrack` or `add` to the position of antenna. Only checking if it is inside a map, if so add it to the list of nodes. Finally return the size of collected nodes.

#### Part 2

In the second part we use similar approach only that we also add the given antennas to the list of nodes. Then we do not calculate just one antinode, but instead loop until we jump out of the map. With the same difference and operation.
