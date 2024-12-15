#### Part 1

In the first part we simulate the robot step by step. If there is `Space` move, if there is a `Wall` stop and if there is a `Box` then try to move all boxes, which is done by following the line of boxing and potentially placing the first box to the end, hence the same operation as if we moved the whole row of boxes.

After these procedures we compute `GPS`.

#### Part 2

For the second part we proceed very similarly. Only change is when dealing with boxes. In a horizontal way we do the same thing, except that we do not put the box to the end, but instead move shift the row of boxes. For the vertical move we will only consider the case that we ran into a left part of the box (otherwise we switch to the left part). Then we check if both parts can be moved or not by recursion. After we check the whole sequence of boxes and get `true` answer we then may move all boxes proceeding in the same matter.
