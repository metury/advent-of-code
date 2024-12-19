package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

const (
	Blue   = "[1;34m"
	Yellow = "[1;33m"
	Green  = "[1;32m"
	Reset  = "[0m"
)

func print_result(dur time.Duration, part, result int) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

func read_file(file_path string) ([]string, []string) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	patterns := strings.Split(lines[0], ", ")
	var rest []string
	for _, line := range lines[1:]{
		if len(line) == 0 {
			continue
		}
		rest = append(rest, line)
	}
	return patterns, rest
}

func is_plaussible(patterns []string, towel string) bool {
	if len(towel) == 0 {
		return true
	}
	plaussible := false
	for _, pattern := range patterns {
		if len(towel) < len(pattern) {
			continue
		}
		if towel[:len(pattern)] == pattern {
			plaussible = plaussible || is_plaussible(patterns, towel[len(pattern):])
		}
		if plaussible {
			return plaussible
		}
	}
	return plaussible
}

func print_out(towel string, m map[[2]int]int) {
	for key, val := range m {
		fmt.Println(towel[key[0]:key[1]], val)
	}
}

func tabelize(patterns map[string]bool, towel string) int {
	results := make(map[[2]int]int)
	for i := 1; i <= len(towel); i++ {
		for j := 0; j < len(towel); j++ {
			e := j + i
			if e > len(towel) {
				continue
			}
			results[[2]int{j, e}] = 0
			_, ok := patterns[towel[j : e]]
			if ok {
				results[[2]int{j, e}] += 1
			}
			for k := j + 1; k < e; k++ {
 				res := results[[2]int{j, k}] * results[[2]int{k,e}]
				results[[2]int{j, e}] += res
			}
		}
	}
	print_out(towel, results)
	fmt.Println()
	return results[[2]int{0, len(towel)}]
}

func nr_is_plaussible(patterns []string, towel string, cache *map[string]int) int {
	if len(towel) == 0 {
		return 1
	}
	plaussible := 0
	for _, pattern := range patterns {
		if len(towel) < len(pattern) {
			continue
		}
		if towel[:len(pattern)] == pattern {
			res, ok := (*cache)[towel[len(pattern):]]
			if ok {
				plaussible += res
			} else {
				r := nr_is_plaussible(patterns, towel[len(pattern):], cache)
				(*cache)[towel[len(pattern):]] = r
				plaussible += r
			}

		}
	}
	return plaussible
}

func part_one() {
	var result int
	start := time.Now()
	patterns, rest := read_file("INPUT")
	for _, tow := range rest {
		if is_plaussible(patterns, tow) {
			result += 1
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	patterns, towels := read_file("INPUT")
	cache := make(map[string]int)
	for _, towel := range towels {
		result += nr_is_plaussible(patterns, towel, &cache)
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "19 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
