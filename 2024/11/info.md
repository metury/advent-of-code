#### Part 1

The first part can be done by brute force, that is going in an array and splitting, changing the numbers. Tho this is not a good approach. See part 2 for more info.

#### Part 2

Generally it is better to hold the number of same stones, since the operation for these stones will be exactly same, so we will instead move all such stones at once. This is under the assumption, that there will be many stones of the same number, because otherwise it will be the same as brute-forcing it. Therefore we create a map for stones as key and we do operation for all keys and add the value to the new number. But it is better to create a new map instead of making it in place.
