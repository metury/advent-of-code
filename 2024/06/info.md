#### Part 1

In the first part we only simulate the steps of the guard and mark visited places on the map. Afterward we count the number of such visited places.

#### Part 2

In the second part we also compute the visited places as in previous step to see candidates for obstacles. Because we may easily see that putting an obstacle outside the path is worthless. Then we put the obstacle into the map and test if we find loop or not. If so count such instance and move on. *Note that this is not close to optimal solution, since we are computing the prefix of the path over and over again. It would be good idea to remember previous path to not compute it again.* On the other hand given solution terminates with the result after a few seconds (approximately 3s), therefore it is not worth the time to optimize the code. **After using `goroutines` I got way faster approach which is efficient enough.**
