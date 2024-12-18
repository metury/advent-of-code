#### Part 1

For the first part we may employ any graph search algorithm. In this solution I created simple recursive DFS algorithm. For such we will note the length of shortest path and the predecessor. In this part we do not need predecessors. After that we check what is the length in the end.

#### Part 2

In the second part we always recreate the path from predecessors and check if the byte will drop onto the path or not. If not we can just put it there and not check the path, if it will drop onto the map we need to recompute the shortest path.

