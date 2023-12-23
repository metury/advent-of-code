#### Part 1

In the first part we parse the input than sort the bricks by their `z` value. After that we simply squish the bricks together (or `compress` them) so they won't  levitate anymore. After that we compute how they are dependent on each other, or in other words how they **support** each other. Lastly we compute these that follows the rules and left out the others.

#### Part 2

We use the already written code to construct such bricks. After that we look on every brick and remove it. Then see if any bricks that were supported by this will fall (so the **interection** with the `supported_by` contains all of them. Then we follow on these bricks until they stay in place.
