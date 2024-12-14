package main

import (
	"fmt"
	"log"
	"os"
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

type Map [][]bool
type Position [2]int

var INVALID_POSITION [2]int = [2]int{-1, -1}

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

func guard_step(m Map, pos, step, obstacle Position) (Position, Position, bool) {
	new_pos := [2]int{pos[0] + step[0], pos[1] + step[1]}
	inside := new_pos[0] < len(m) && new_pos[0] >= 0 && new_pos[1] < len(m[0]) && new_pos[1] >= 0
	if inside {
		in_map := m[new_pos[0]][new_pos[1]] && (new_pos[0] != obstacle[0] || new_pos[1] != obstacle[1])
		if in_map {
			return new_pos, step, true
		} else {
			step[0], step[1] = step[1], -step[0]
			return pos, step, true
		}
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
	pos, step, ok := guard_step(m, pos, step, INVALID_POSITION)
	for ok {
		visited[pos[0]][pos[1]] = true
		pos, step, ok = guard_step(m, pos, step, INVALID_POSITION)
	}
	return visited
}

func is_loop(m Map, pos, obstacle Position, c chan bool) {
	visited := make(map[[4]int]bool)
	step := [2]int{-1, 0}
	visited[[4]int{pos[0], pos[1], step[0], step[1]}] = true
	pos, step, ok := guard_step(m, pos, step, obstacle)
	hash_ok := false
	for ok && !hash_ok {
		_, hash_ok = visited[[4]int{pos[0], pos[1], step[0], step[1]}]
		visited[[4]int{pos[0], pos[1], step[0], step[1]}] = true
		pos, step, ok = guard_step(m, pos, step, obstacle)
	}
	c <- hash_ok
}

func part_one() {
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
	print_result(end.Sub(start), 1, result)
}

func part_two() {
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
	c := make(chan bool)
	for _, obstacle := range visited_pos {
		go is_loop(m, pos, obstacle, c)
	}
	for i := 0; i < len(visited_pos); i++ {
		if <-c {
			result += 1
		}
	}
	close(c)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "6 - Guard Gallivant" + Reset)
	part_one()
	part_two()
}
