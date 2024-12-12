package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"time"
)

func read_file(file_path string) string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}

func part_one() {
	var result int
	var content = read_file("INPUT")
	start := time.Now()
	regex := regexp.MustCompile("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)")
	found := regex.FindAllStringSubmatch(content, -1)
	for _, f := range found {
		nr1, _ := strconv.Atoi(f[1])
		nr2, _ := strconv.Atoi(f[2])
		result += nr1 * nr2
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	content := read_file("INPUT")
	start := time.Now()
	regex := regexp.MustCompile("do\\(\\)|don't\\(\\)|mul\\(([0-9]{1,3}),([0-9]{1,3})\\)")
	found := regex.FindAllStringSubmatch(content, -1)
	enabled := true
	for _, f := range found {
		switch f[0] {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if enabled {
				nr1, _ := strconv.Atoi(f[1])
				nr2, _ := strconv.Atoi(f[2])
				result += nr1 * nr2
			}
		}
	}
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 3 - Mull It Over")
	part_one()
	part_two()
}
