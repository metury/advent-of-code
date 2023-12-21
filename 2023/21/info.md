#### Part 1

For the first part we have more options.

1. Completely brute force it. It is just `64` steps. **:D**
2. Remember two sets: Odd visited and even visited. Then see the number of even visited with the limit `64`.
3. Use `bfs` algorithm to compute distances to every place. Then just filter out these that were visited in an even steep and is under the limit of `64`.

#### Part 2

The second part uses the implemented `bfs`. We are not able to compute it for all `26501365` steps. But there is some quite nice math behind the input. Firstly it is a square and there is a straight line from the middle to each side. Every neighbouring blocks switch **even**/**odd** parity. And lastly we campute the corners.

This is just simplified steps of the computation. I actually didn't come up with that on my own, but one can see it make sense due to the nice input. By which we can have some assumptions.
