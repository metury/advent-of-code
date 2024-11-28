#!/bin/bash

set -ueo pipefail

for year in *; do
	if [ -d "$year" ]; then
		cd "$year"
		for day in *; do
			if [ -d "$day" ]; then
				cd "$day"
				if [ -f "Cargo.toml" ]; then
					cargo fmt
				fi
				cd ..
			fi
		done
		cd ..
	fi
done
