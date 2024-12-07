package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

type Equation struct {
	result   int
	operands []int
}

func read_file(file_path string) []Equation {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	var equations []Equation
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		parts := strings.Split(line, " ")
		result, _ := strconv.Atoi(parts[0][:len(parts[0])-1])
		var numbers []int
		for _, part := range parts[1:] {
			nr, _ := strconv.Atoi(part)
			numbers = append(numbers, nr)
		}
		equations = append(equations, Equation{result, numbers})
	}
	return equations
}

func concat(x int, y int) int {
	res, _ := strconv.Atoi(strconv.Itoa(x) + strconv.Itoa(y))
	return res
}

func solve(eq Equation, acc int, i int) bool {
	if i < len(eq.operands) && acc > eq.result {
		return false
	}
	if i == len(eq.operands) {
		return eq.result == acc
	}
	return solve(eq, acc+eq.operands[i], i+1) || solve(eq, acc*eq.operands[i], i+1)
}

func solve_concat(eq Equation, acc int, i int) bool {
	if i < len(eq.operands) && acc > eq.result {
		return false
	}
	if i == len(eq.operands) {
		return eq.result == acc
	}
	return solve_concat(eq, acc+eq.operands[i], i+1) || solve_concat(eq, acc*eq.operands[i], i+1) || solve_concat(eq, concat(acc, eq.operands[i]), i+1)
}

func common_solve(eq Equation, solver func(Equation, int, int) bool, c chan int) {
	if solver(eq, 0, 0) {
		c <- eq.result
	} else {
		c <- 0
	}
}

func part1() {
	var result int
	start := time.Now()
	equations := read_file("INPUT")
	c := make(chan int)
	for _, eq := range equations {
		go common_solve(eq, solve, c)
	}
	for i := 0; i < len(equations); i++ {
		result += <-c
	}
	close(c)
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	equations := read_file("INPUT")
	c := make(chan int)
	for _, eq := range equations {
		go common_solve(eq, solve_concat, c)
	}
	for i := 0; i < len(equations); i++ {
		result += <-c
	}
	close(c)
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 7 - Bridge Repair")
	part1()
	part2()
}
