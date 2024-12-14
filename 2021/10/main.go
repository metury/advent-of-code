package main

import (
	"fmt"
	"log"
	"os"
	"sort"
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
	print_result(end.Sub(start), 1, result)
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
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2021" + Reset + " day " + Green + "10 - Syntax Scoring" + Reset)
	part_one()
	part_two()
}
