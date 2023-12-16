#!/bin/bash

set -ueo pipefail

## ==================== ##
##  Constants           ##
## ==================== ##

aoc="advent of code"
link="/aoc/"
aoc_dir="aoc"
aoc_file="adventofcode.md"
forbidden_files="INPUT OUTPUT Cargo.toml Cargo.lock info.md main"

## ==================== ##
##  Functions           ##
## ==================== ##

# Print the title page.
print_aoc(){
	echo "---
layout: page
title: $aoc
permalink: $link
has_children: true
---

They are my solutions to [advent of code](https://adventofcode.com/) tasks. There are separated to each year and day. All of this can be found on [GitHub](https://github.com/metury/advent-of-code), also with the script that generates these pages.

Plus you may also play a small [Bingo](https://aoc-bingo.fly.dev/) that someone made. Also you may consider joining [Reddit](https://www.reddit.com/r/adventofcode/) where you may find useful tips, or help someone.

### Years
" > "$aoc_file"
}

# Print year page by: number.
print_year(){
	echo "---
layout: page
title: Year $1
parent: $aoc
permalink: $link$1/
has_children: true
---

This contains tasks from the year $dir.

### Days
" > ../"$aoc_dir"/"$1".md
}

# Print day by its number and number of year.
print_day(){
	echo "---
layout: page
title: Day $1
parent: Year $2
grand_parent: $aoc
permalink: $link$2/$1/
---

This is a solution of the day $1.
" > "$3"
}

# File, and where to print it.
print_file(){
	echo "## $1" >> "$2"
	echo "" >> "$2"
	echo "\`\`\`$(echo "$1" | cut -d. -f2)" >> "$2"
	expand -i -t2 "$1" >> "$2"
	echo "\`\`\`" >> "$2"
}

## ==================== ##
##  Main loop           ##
## ==================== ##

mkdir -p "$aoc_dir"
print_aoc

for dir in *; do
	if [[ "$dir" == "$aoc_dir" ]]; then
		continue
	fi
	if [ -d "$dir" ]; then
		echo "- [Year $dir]($link$dir)" >> "$aoc_file"
		cd "$dir"
		print_year "$dir"
		for subdir in *; do
			if [ -d "$subdir" ]; then
				echo "- [Day $subdir]($link$dir/$subdir/)" >> ../"$aoc_dir"/"$dir".md
				cd "$subdir" # move to the inner directory
				out="../../$aoc_dir/$dir-$subdir.md"
				print_day "$subdir" "$dir" "$out"
				for file in *; do
					useful=1
					for forb in $forbidden_files; do
						if [[ "$file" == "$forb" ]]; then
							useful=0
						fi
					done
					if [[ "$file" == "info.md" ]]; then
						echo "### Info" >> "$out"
						echo "" >> "$out"
						cat "$file" >> "$out"
						echo >> "$out"
					fi
					if [[ $useful == 1 ]]; then
						if [ -d "$file" ]; then
							if [ "$file" == "src" ]; then
								cd "$file"
								for src in *; do
									print_file "$src" "../$out"
								done
								cd ..
							fi
						else
							print_file "$file" "$out"
						fi
					fi
				done
				cd ..
			fi
		done
		cd ..
	fi
done
