#!/bin/bash


# Create a templated directory for info.md and INPUT.
# Arguments:
#  1 Year
#  2 Day

set -ueo pipefail

if [ $# -lt 2 ]; then
	echo "Two arguments were not found. Year and day must be provided."
	exit
fi

year=$1
day=$2

mkdir -p "$year"
cd "$year"
mkdir -p "$day"
cd "$day"

touch "INPUT"
touch "info.md"

cd ../..

