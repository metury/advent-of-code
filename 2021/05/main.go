package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"strconv"
	"time"
)

type Line struct {
	start [2]int
	end   [2]int
}

func read_file(file_path string) []Line {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex := regexp.MustCompile("([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)")
	found := regex.FindAllStringSubmatch(string(content), -1)
	var lines []Line
	for _, f := range found {
		start_x, _ := strconv.Atoi(f[1])
		start_y, _ := strconv.Atoi(f[2])
		end_x, _ := strconv.Atoi(f[3])
		end_y, _ := strconv.Atoi(f[4])
		lines = append(lines, Line{[2]int{start_x, start_y}, [2]int{end_x, end_y}})
	}
	return lines
}

func add_to_map(number_of_lines *map[[2]int]int, pos [2]int) {
	_, ok := (*number_of_lines)[pos]
	if ok {
		(*number_of_lines)[pos] += 1
	} else {
		(*number_of_lines)[pos] = 1
	}
}

func midpoints(number_of_lines *map[[2]int]int, line Line) {
	diff := [2]int{line.start[0] - line.end[0], line.start[1] - line.end[1]}
	if diff[0] == 0 {
		incr := diff[1] / int(math.Abs(float64(diff[1])))
		for x := line.end[1]; x != line.start[1]; x += incr {
			add_to_map(number_of_lines, [2]int{line.start[0], x})
		}
		add_to_map(number_of_lines, line.start)
	} else if diff[1] == 0 {
		incr := diff[0] / int(math.Abs(float64(diff[0])))
		for x := line.end[0]; x != line.start[0]; x += incr {
			add_to_map(number_of_lines, [2]int{x, line.start[1]})
		}
		add_to_map(number_of_lines, line.start)
	} else if int(math.Abs(float64(diff[1]))) == int(math.Abs(float64(diff[0]))) {
		incr := [2]int{diff[0] / int(math.Abs(float64(diff[0]))), diff[1] / int(math.Abs(float64(diff[1])))}
		y := line.end[1]
		for x := line.end[0]; x != line.start[0]; x += incr[0] {
			add_to_map(number_of_lines, [2]int{x, y})
			y += incr[1]
		}
		add_to_map(number_of_lines, line.start)

	}
}

func part_one() {
	var result int
	start := time.Now()
	number_of_lines := make(map[[2]int]int)
	lines := read_file("INPUT")
	for _, line := range lines {
		if line.start[0] == line.end[0] || line.start[1] == line.end[1] {
			midpoints(&number_of_lines, line)
		}
	}
	for _, ele := range number_of_lines {
		if ele >= 2 {
			result += 1
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	start := time.Now()
	number_of_lines := make(map[[2]int]int)
	lines := read_file("INPUT")
	for _, line := range lines {
		midpoints(&number_of_lines, line)
	}
	for _, ele := range number_of_lines {
		if ele >= 2 {
			result += 1
		}
	}
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2021 day 5 - Hydrothermal Venture")
	part_one()
	part_two()
}
