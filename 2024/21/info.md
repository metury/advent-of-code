#### Part 1

Generally we will establish ordering of arrow symbols, in this order `<`, `v`, `^`, `>` and `A`. Then we will create a mapping from one sequence on a pad to all others following te instructions. These sequences will be sorted in a previously mentioned way. Also we will move all leading `>` symbols (if there is more than 1 of them). This mapping is from two characters to a sequence of instructions.

Then we will create optimal solution for numerical pad and then we will keep in map number of common sub-strings neighboring with `A`'s. We will repair such sequence by simulating the movement on the pad. lastly we move all symbols so that the same ones are neighbouring.

This way we proceed for the directional pad and compute it for `2` repetitions.

#### Part 2

In the second one we only change `2` to `25`.

