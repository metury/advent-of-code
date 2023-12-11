#!/bin/bash

set -ueo pipefail

aoc="advent of code"
link="/aoc/"
aoc_dir="aoc"
aoc_file="adventofcode.md"

mkdir -p "$aoc_dir"

echo "---
layout: page
title: $aoc
permalink: $link
has_children: true
---

They are my solutions to [advent of code](https://adventofcode.com/) tasks. There are separated to each year and day. All of this can be found on [GitHub](https://github.com/metury/advent-of-code), also with the script that generates these pages.

Plus you may also play a small [Bingo](https://aoc-bingo.fly.dev/) that someone made. :D

" > "$aoc_file"

for dir in *; do
	if [[ "$dir" == "$aoc_dir" ]]; then
		continue
	fi
	if [ -d "$dir" ]; then
		echo "- [Year $dir]($link$dir)" >> "$aoc_file"
		cd "$dir" # move to the top level directory.
		echo "---
layout: page
title: Year $dir
parent: $aoc
permalink: $link$dir/
has_children: true
---

This contains tasks from the year $dir.

" > ../"$aoc_dir"/"$dir".md
		for subdir in *; do
			if [ -d "$subdir" ]; then
				echo "- [Day $subdir]($link$dir/$subdir/)" >> ../"$aoc_dir"/"$dir".md
				cd "$subdir" # move to the inner directory
				out="../../$aoc_dir/$dir-$subdir.md"
				echo "---
layout: page
title: Day $subdir
parent: Year $dir
grand_parent: $aoc
permalink: $link$dir/$subdir/
---

This is a solution of the day $subdir.

" > "$out"
				for file in *; do
					if [ "$file" != "INPUT" ] && [ "$file" != "main" ]; then
						echo "# $file" >> "$out"
						echo "" >> "$out"
						echo "\`\`\`$(echo "$file" | cut -d. -f2)" >> "$out"
						expand -i -t2 "$file" >> "$out"
						echo "\`\`\`" >> "$out"
						echo "" >> "$out"
					fi
				done
				cd ..
			fi
		done
		cd ..
	fi
done