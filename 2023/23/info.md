#### Part 1

First part is pretty simple and fast. We just recursively go through the map and follow the slopes. We are trying every possibility and then find the maximum.

#### Part 2

For the second part they are at least two approaches.

1. Use the same code just ignore the slopes and try every possibilty. As one may see this is not quite the best way to do it. But as it turns out you will wait way less than an hour, so it si doable.
2. The second more complex way is to actually compress each path and create a simplified graph. Then again try every possibility. it will still take probably some time, but there is less vertices and edges to consider.

In my code I just used the brute force, because after a lunch the result was already processed.

