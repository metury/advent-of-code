package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strings"
	"time"
)

func read_file(file_path string) []string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	for i := 0; i < len(lines); i++ {
		if len(lines[i]) == 0 {
			lines = append(lines[:i], lines[i+1:]...)
			i--
		}
	}
	return lines
}

func filter_or_fill(line string) (int, int) {
	var stack []byte
	for i := 0; i < len(line); i++ {
		switch line[i] {
		case '<', '(', '{', '[':
			stack = append(stack, line[i])
		case '>', ')', '}', ']':
			top := stack[len(stack)-1]
			stack = stack[:len(stack)-1]
			if line[i] == '>' && top != '<' {
				return 25137, 0
			} else if line[i] == ')' && top != '(' {
				return 3, 0
			} else if line[i] == ']' && top != '[' {
				return 57, 0
			} else if line[i] == '}' && top != '{' {
				return 1197, 0
			}
		}
	}
	sum := 0
	for len(stack) > 0 {
		top := stack[len(stack)-1]
		stack = stack[:len(stack)-1]
		sum *= 5
		switch top {
		case '(':
			sum += 1
		case '[':
			sum += 2
		case '{':
			sum += 3
		case '<':
			sum += 4
		}
	}
	return 0, sum
}

func part_one() {
	var result int
	start := time.Now()
	lines := read_file("INPUT")
	for _, line := range lines {
		val, _ := filter_or_fill(line)
		result += val
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	start := time.Now()
	lines := read_file("INPUT")
	var scores []int
	for _, line := range lines {
		_, val := filter_or_fill(line)
		if val != 0 {
			scores = append(scores, val)
		}
	}
	sort.Ints(scores)
	result = scores[len(scores)/2]
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2021 day 10 - Syntax Scoring")
	part_one()
	part_two()
}
