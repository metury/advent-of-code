#### Part 1

We will always create an array of arrays. Then we create a closure which will remember the last state. Then we always check whether it is inside bounds for the limit and also if it is increasing or decreasing. For those we wil have two closures. Later on it was rewritten into one closure.

#### Part 2

For the second part I added toleration. That is we can skip one element **inside** the array. But we have to take care also about the case where *the first element* can be omitted, therefore I will call two closures. Hence it is not bruteforcing all possible missing indices, but I am considering only two cases.

