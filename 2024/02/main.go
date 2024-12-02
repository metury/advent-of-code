package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

const LIMIT int = 3

func read_file(file_path string) [][]int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var res [][]int
	splited := strings.Split(string(content), "\n")
	for i, el := range splited {
		level := strings.Split(el, " ")
		if len(el) == 0 {
			continue
		}
		res = append(res, make([]int, len(level)))
		for j, val := range level {
			res[i][j], _ = strconv.Atoi(val)
		}
	}
	return res
}

func comparing(tolerate bool, start int, compare func(int, int) bool) func(int) bool{
	current := start
	last := true
	tol := tolerate
	return func(x int) bool {
		diff := int(math.Abs(float64(current - x)))
		if tol && (!compare(x, current) || diff > LIMIT) {
			tol = false
		} else {
			last = compare(x, current) && last && diff <= LIMIT
			current = x
		}
		return last
	}
}

func increasing(tolerate bool, start int) func(int) bool {
	greeater_than := func(x, y int) bool {
		return x > y
	}
	return comparing(tolerate, start - 1, greeater_than)
}

func decreasing(tolerate bool, start int) func(int) bool {
	less_than := func(x, y int) bool {
		return x < y
	}
	return comparing(tolerate, start + 1, less_than)
}

func part1() {
	var result int
	levels := read_file("INPUT")
	for _, level := range levels {
		i := increasing(false, level[0])
		d := decreasing(false, level[0])
		incr, decr := true, true
		for _, val := range level {
			incr = incr && i(val)
			decr = decr && d(val)
		}
		if incr || decr {
			result += 1
		}
	}
	fmt.Println("Part 1: ", result)
}

func part2() {
	var result int
	levels := read_file("INPUT")
	for _, level := range levels {
		i, i_without_first := increasing(true, level[0]), increasing(false, level[1])
		d, d_without_first := decreasing(true, level[0]), decreasing(false, level[1])
		incr, decr, incr_without_first, decr_without_first := true, true, true, true
		for _, val := range level {
			incr = incr && i(val)
			decr = decr && d(val)
		}
		for _, val := range level[1:] {
			incr_without_first = incr_without_first && i_without_first(val)
			decr_without_first = decr_without_first && d_without_first(val)
		}
		if incr || decr || incr_without_first || decr_without_first {
			result += 1
		}
	}
	fmt.Println("Part 2: ", result)
}

func main() {
	fmt.Println("Year 2024 day 2 - Red-Nosed Reports")
	part1()
	part2()
}
