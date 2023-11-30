#!/bin/bash

set -ueo pipefail

aoc="Advent of code"
link="/aoc/"
aoc_dir="aoc"

mkdir -p "$aoc_dir"

echo "---
layout: page
title: $aoc
permalink: $link
has_children: true
---

# [Advent of code](https://adventofcode.com/)

There are my solutions to advent of code tasks. There are separated to each year and day. All of this can be found on [GitHub](https://github.com/metury/advent-of-code), also with the script that generates these pages.
" > "adventofcode.md"

for dir in *; do
	if [[ "$dir" == "$aoc_dir" ]]; then
		continue
	fi
	if [ -d "$dir" ]; then
		cd "$dir" # move to the top level directory.
		echo "---
layout: page
title: Year $dir
parent: $aoc
permalink: $link$dir/
has_children: true
---

# Year $dir

This contains tasks from the year $dir.

" > ../"$aoc_dir"/"$dir".md
		for subdir in *; do
			if [ -d "$subdir" ]; then
				cd "$subdir" # move to the inner directory
				out="../../$aoc_dir/$dir-$subdir.md"
				echo "---
layout: page
title: Day $subdir
parent: Year $dir
grand_parent: $aoc
permalink: $link$dir/$subdir/
---

# Day $subdir

This is a solution of the day $subdir.
" > "$out"
				for file in *; do
					echo "# $file" >> "$out"
					echo "" >> "$out"
					echo "\`\`\`$(echo "$file" | cut -d. -f2)" >> "$out"
					cat "$file" >> "$out"
					echo "\`\`\`" >> "$out"
					echo "" >> "$out"
				done
				cd ..
			fi
		done
		cd ..
	fi
done