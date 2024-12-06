package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

type Map [][]bool
type Position [2]int

func read_file(file_path string) (Map, Position) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	var m (Map)
	var pos Position
	for i, line := range lines {
		if len(line) <= 0 {
			continue
		}
		row := make([]bool, len(line))
		for j, char := range strings.Split(line, "") {
			switch char {
			case ".":
				row[j] = true
			case "#":
				row[j] = false
			case "^":
				pos = [2]int{i, j}
				row[j] = true
			}
		}
		m = append(m, row)
	}
	return m, pos
}

func guard_step(m Map, pos Position, step Position) (Position, Position, bool) {
	new_pos := [2]int{pos[0] + step[0], pos[1] + step[1]}
	inside := new_pos[0] < len(m) && new_pos[0] >= 0 && new_pos[1] < len(m[0]) && new_pos[1] >= 0
	if inside && m[new_pos[0]][new_pos[1]] {
		return new_pos, step, inside
	} else if inside {
		step[0], step[1] = step[1], -step[0]
		inside := pos[0] < len(m) && pos[0] >= 0 && pos[1] < len(m[0]) && pos[1] >= 0
		return pos, step, inside
	} else {
		return new_pos, step, false
	}
}

func found_visited(m Map, pos Position) Map {
	visited := make(Map, len(m))
	for i := range visited {
		visited[i] = make([]bool, len(m[i]))
	}
	visited[pos[0]][pos[1]] = true
	step := [2]int{-1, 0}
	pos, step, ok := guard_step(m, pos, step)
	for ok {
		visited[pos[0]][pos[1]] = true
		pos, step, ok = guard_step(m, pos, step)
	}
	return visited
}

func is_loop(m Map, pos Position) bool {
	visited := make(map[[4]int]bool)
	step := [2]int{-1, 0}
	visited[[4]int{pos[0], pos[1], step[0], step[1]}] = true
	pos, step, ok := guard_step(m, pos, step)
	_, hash_ok := visited[[4]int{pos[0], pos[1], step[0], step[1]}]
	for ok && !hash_ok {
		_, hash_ok = visited[[4]int{pos[0], pos[1], step[0], step[1]}]
		visited[[4]int{pos[0], pos[1], step[0], step[1]}] = true
		pos, step, ok = guard_step(m, pos, step)
	}
	return hash_ok
}

func part1() {
	var result int
	start := time.Now()
	m, pos := read_file("INPUT")
	visited := found_visited(m, pos)
	for i := range visited {
		for j := range visited[i] {
			if visited[i][j] {
				result += 1
			}
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	m, pos := read_file("INPUT")
	visited_map := found_visited(m, pos)
	var visited_pos [][2]int
	for i := range visited_map {
		for j := range visited_map[i] {
			if visited_map[i][j] {
				visited_pos = append(visited_pos, [2]int{i, j})
			}
		}
	}
	for _, obstacle := range visited_pos {
		m[obstacle[0]][obstacle[1]] = false
		if is_loop(m, pos) {
			result += 1
		}
		m[obstacle[0]][obstacle[1]] = true
	}

	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 6 - Guard Gallivant")
	part1()
	part2()
}
