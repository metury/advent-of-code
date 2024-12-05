#### Part 1

For both cases we read the file via regex and filter out the wrong sequences and good sequences. That is for each page in single order we check previous pages if they violate some rules. Then we just compute the sum of middle pages.

#### Part 2

For the second part we consider only the incorrect ones. We go by pages and check their violations. If we find some we put the vilated pages behind the current page while removing the violated ones. And proceed by the corrected page onwards, because the pages inserted afterward may be wrong.


```txt
-----------
| |X| |O| |
-----------
   ^   ^
   j   k

---------
| | |O| |
---------
   ^ ^
   j k


-----------
| | |O|X| |
-----------
	 ^
	 k
```
