#### Part 1

In the first part we do a `BFS` and mark visited places indexed by place and with value of their length. If we may step on the tile with smaller score do so.

#### Part 2

In the second part we also return all visited places with the shortest path, but now we recourse on the path which has also the same score. Also if we run out of the budget, which was computed from the previous step we end.

