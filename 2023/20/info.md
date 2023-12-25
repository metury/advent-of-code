#### Part 1

In the first part you run the bfs to firstly find predecessors. Then you have the whole graph represented. After you run once again bfs with some different cases for the logic to work.

#### Part 2

The second part is easier to solve by hand. You may create a `graph.dot` in [graphviz dot language](https://graphviz.org/doc/info/lang.html) and then by running `dot -Kdot -Tsvg graph.dot -o graph.svg` create a picture of the graph. There are 4 modules that behave like a binary counters. For each you represent it by a binary number; 1 if the edge goes to the conjection and 0 otherwise. After you represent it you use LCM for them and that is the answer.
