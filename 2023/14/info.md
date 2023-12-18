#### Part 1

In the first part we create a grid of all stones and shift it to one north side and then compute it.

#### Part 2

In the second part we cycle the shifts counterclockwis and always remember current number of cycle and how the map looked. If we encounter the same situation we take a shortcut by skipping all repeating cycles and only finish the last ones.
