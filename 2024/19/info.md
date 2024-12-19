#### Part 1

For the first part we try a prefix if there is a matching pattern, if so recourse on the suffix. If not return false. If we finally found some, terminate the process.

#### Part 2

The second part is done pretty much the same way, except for counting the number of plausible. Since it would be slow we create a cache for already computed sequences.
