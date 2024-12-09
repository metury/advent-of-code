#### Part 1

Today I used two different approaches for each part. In the first part I recreated the disc and by following two indices (going forward and backward) I swap spaces and files between each other. Lastly I compute the sum.

#### Part 2

For the second part I created a structure that holds the content of the place, its size and possibly index of a file. Then I also go backwards and forward at the same time and if I find a file I will try to find a big enough space and then put it there. Afterwards I partition the space if there is some left and compact consecutive spaces to just one.
