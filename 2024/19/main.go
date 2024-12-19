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
	var towels []string
	for _, line := range lines[1:] {
		if len(line) == 0 {
			continue
		}
		towels = append(towels, line)
	}
	return patterns, towels
}

func is_plausible(patterns []string, towel string) bool {
	if len(towel) == 0 {
		return true
	}
	plausible := false
	for _, pattern := range patterns {
		length := len(pattern)
		if len(towel) < length {
			continue
		}
		if towel[:length] == pattern {
			plausible = plausible || is_plausible(patterns, towel[length:])
		}
		if plausible {
			return plausible
		}
	}
	return plausible
}

func nr_of_plausible(patterns []string, towel string, cache *map[string]int) int {
	if len(towel) == 0 {
		return 1
	}
	counter := 0
	for _, pattern := range patterns {
		length := len(pattern)
		if len(towel) < length {
			continue
		}
		if towel[:length] == pattern {
			precomputed, ok := (*cache)[towel[length:]]
			if ok {
				counter += precomputed
			} else {
				computed := nr_of_plausible(patterns, towel[length:], cache)
				(*cache)[towel[length:]] = computed
				counter += computed
			}
		}
	}
	return counter
}

func part_one() {
	var result int
	start := time.Now()
	patterns, rest := read_file("INPUT")
	for _, tow := range rest {
		if is_plausible(patterns, tow) {
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
		result += nr_of_plausible(patterns, towel, &cache)
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "19 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
