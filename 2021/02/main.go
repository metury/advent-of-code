package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
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

type Instruction struct {
	instruction string
	value       int
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
			forward += instruction.value
			depth += aim * instruction.value
		}
	}
	return depth * forward
}

func part_one() {
	var result int
	start := time.Now()
	instructions := read_file("INPUT")
	result = compute(instructions)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	instructions := read_file("INPUT")
	result = compute_aim(instructions)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2021" + Reset + " day " + Green + "2 - Dive!" + Reset)
	part_one()
	part_two()
}
