#### Part 1

In the first part we will go through all nodes and look at their neighbours. For every neighbour we try to add additional neighbour and check if he is adjacent to the original node. This way we add all \\(3!\\) permutation, so we also divide it by `6`.

#### Part 2

For the second part we use pretty much the same approach. We will be keeping the largest clique found so far. Then we look at the neighbours from one node to find a candidate, if the candidate is adjacent to all added nodes so far, then we just add it as well. This way we find the maximal one.

