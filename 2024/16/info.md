#### Part 1

In the first part we do a `BFS` and mark visited places indexed by place and orientation and with value of their length. If we may step on the tile with smaller score do so.

#### Part 2

In the second part we also return all visited places with the shortest path, but now we recourse on the path which has also the same score. Note that this is actually pretty slow, because we are rerunning the same suffix of each shortest path multiple times.

