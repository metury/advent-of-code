package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

func read_file(file_path string) []string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	for i, line := range lines {
		if len(line) == 0 {
			lines = append(lines[:i], lines[i+1:]...)
		}
	}
	return lines
}

func number_of_ones(lines []string, index int) int {
	number_of_ones := 0
	for _, line := range lines {
		if line[index] == '0' {
			number_of_ones -= 1
		} else {
			number_of_ones += 1
		}
	}
	return number_of_ones
}

func significants(lines []string) int {
	gamma, epsilon := 0, 0
	acc := 1
	for j := len(lines[0]) - 1; j >= 0; j-- {
		if number_of_ones(lines, j) > 0 {
			gamma += acc
		} else {
			epsilon += acc
		}
		acc *= 2
	}
	return gamma * epsilon
}

func filter(lines []string, rule func(int, byte) bool) int {
	for i := 0; i < len(lines[0]); i++ {
		number_of_ones := number_of_ones(lines, i)
		for j := 0; j < len(lines); j++ {
			if rule(number_of_ones, lines[j][i]) {
				lines = append(lines[:j], lines[j+1:]...)
				j--
			}
		}
		if len(lines) == 1 {
			nr, _ := strconv.ParseInt(lines[0], 2, 64)
			return int(nr)
		}
	}
	return 0
}

func oxygen(lines []string) int {
	rule := func(number_of_ones int, bit byte) bool {
		return (number_of_ones >= 0 && bit == '0') || (number_of_ones < 0 && bit == '1')
	}
	return filter(lines, rule)
}

func co2(lines []string) int {
	rule := func(number_of_ones int, bit byte) bool {
		return (number_of_ones < 0 && bit == '0') || (number_of_ones >= 0 && bit == '1')
	}
	return filter(lines, rule)
}

func part_one() {
	var result int
	start := time.Now()
	lines := read_file("INPUT")
	result = significants(lines)
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	start := time.Now()
	result = oxygen(read_file("INPUT")) * co2(read_file("INPUT"))
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2021 day 3 - Binary Diagnostic")
	part_one()
	part_two()
}
