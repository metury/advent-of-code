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

func create_table(patterns []string) map[string]int{
	dyn := make(map[string]int)
	for _, pat := range patterns {
		dyn[pat] = 1
	}
	max := 2
	forb := make(map[string]bool)
	for  {
		for key, val := range dyn {
			for key2, val2 := range dyn {
				_, ok := dyn[key + key2]
				_, f := forb[key+key2]
				if ok && !f{
					dyn[key+key2] += val*val2
				}
				_, ok = dyn[key2 + key]
				_, f = forb[key2+key]
				if ok && !f && key != key2{
					dyn[key2+key] += val*val2
				}
				forb[key+key2] = true
				forb[key2+key] = true
			}
		}
		max++
	}
	return dyn
}

func nr_is_plaussible(patterns []string, towel string) int {
	if len(towel) == 0 {
		return 1
	}
	plaussible := 0
	for _, pattern := range patterns {
		if len(towel) < len(pattern) {
			continue
		}
		if towel[:len(pattern)] == pattern {
			plaussible += nr_is_plaussible(patterns, towel[len(pattern):])
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
	patterns, _ := read_file("INPUT")
	dyn := create_table(patterns)
	fmt.Println(dyn)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "19 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
