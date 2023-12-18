#### Part 1

We can simply calculate the whole grid. Then create the outline and after that recursively fill in the polygon from a starting node. This solution is actually in here anymore.

#### Part 2

Again we use [Pick's Theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem). And [geo](https://docs.rs/geo/latest/geo/) library to create a polygon from the instruction set and compute the area using the library. Then we procede with the use of the theoem.
