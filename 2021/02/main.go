package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"time"
)

type Instruction struct{
	instruction string
	value int
}

func read_file(file_path string) []Instruction {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex := regexp.MustCompile("(forward|up|down) ([0-9])")
	found := regex.FindAllStringSubmatch(string(content), -1)
	instructions := make([]Instruction, len(found))
	for i, f := range found {
		nr, _ := strconv.Atoi(f[2])
		instructions[i] = Instruction{f[1], nr}
	}
	return instructions
}

func compute(instructions []Instruction) int {
	depth, forward := 0, 0
	for _, instruction := range instructions {
		switch instruction.instruction {
			case "up":
				depth -= instruction.value
			case "down":
				depth += instruction.value
			case "forward":
				forward += instruction.value
		}
	}
	return depth * forward
}

func compute_aim(instructions []Instruction) int {
	depth, forward, aim := 0, 0, 0
	for _, instruction := range instructions {
		switch instruction.instruction {
			case "up":
				aim -= instruction.value
			case "down":
				aim += instruction.value
			case "forward":
				forward += instruction.value;
				depth += aim * instruction.value;
		}
	}
	return depth * forward
}

func part1() {
	var result int
	start := time.Now()
	instructions := read_file("INPUT")
	result = compute(instructions)
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	instructions := read_file("INPUT")
	result = compute_aim(instructions)
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2021 day 2 - Dive!")
	part1()
	part2()
}
