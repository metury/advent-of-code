package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
	"time"
)

const (
	Blue   = "\033[1;34m"
	Yellow = "\033[1;33m"
	Green  = "\033[1;32m"
	Reset  = "\033[0m"
)

func print_result(dur time.Duration, part, result int) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

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

func comparing(tolerate bool, start int, compare func(int, int) bool) func(int) bool {
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
	return comparing(tolerate, start-1, greeater_than)
}

func decreasing(tolerate bool, start int) func(int) bool {
	less_than := func(x, y int) bool {
		return x < y
	}
	return comparing(tolerate, start+1, less_than)
}

func increasing_or_decreasing(tolerate bool, level *[]int) bool {
	i := increasing(tolerate, (*level)[0])
	d := decreasing(tolerate, (*level)[0])
	incr, decr := true, true
	for _, val := range *level {
		incr = incr && i(val)
		decr = decr && d(val)
	}
	return incr || decr
}

func part_one() {
	var result int
	start := time.Now()
	levels := read_file("INPUT")
	for _, level := range levels {
		if increasing_or_decreasing(false, &level) {
			result += 1
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	levels := read_file("INPUT")
	for _, level := range levels {
		level_without_first := level[1:]
		if increasing_or_decreasing(true, &level) || increasing_or_decreasing(false, &level_without_first) {
			result += 1
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "2 - Red-Nosed Reports" + Reset)
	part_one()
	part_two()
}
