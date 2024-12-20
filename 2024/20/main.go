package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

const (
	Blue   = "[1;34m"
	Yellow = "[1;33m"
	Green  = "[1;32m"
	Reset  = "[0m"
)

func print_result(dur time.Duration, part, result int) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

const Max int = int(^uint(0) >> 1)

func read_file(file_path string) ([][]bool, [2]int, [2]int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var start [2]int
	var end [2]int
	var m [][]bool
	for i, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		row := make([]bool, len(line))
		for j, c := range line {
			switch c {
				case '#':
					row[j] = false
				case '.':
					row[j] = true
				case 'S':
					row[j] = true
					start = [2]int{i, j}
				case 'E':
					row[j] = true
					end = [2]int{i, j}
			}
		}
		m = append(m, row)
	}
	return m, start, end
}

func get_neighbours(x, y int) [4][2]int {
	 return [4][2]int{{x + 1,y}, {x - 1, y}, {x, y + 1},{x, y - 1}}
}

func find_shortest_path(m *[][]bool, position [2]int, length int, visited *[][]int) {
	if position[0] < 0 || position[1] < 0 || position[0] >= len(*m) || position[1] >= len((*m)[position[0]]) {
		return
	}
	if !(*m)[position[0]][position[1]] || (*visited)[position[0]][position[1]] < length {
		return
	}
	(*visited)[position[0]][position[1]] = length
	for _, neigh := range get_neighbours(position[0], position[1]) {
		find_shortest_path(m, neigh, length + 1, visited)
	}
}

func find_shortcuts(m *[][]bool, visited *[][]int, timer, limit, length int, position [2]int, previous map[[2]int]int, ends map[[2]int]bool) {
	timer--
	length++
	if timer < 0 {
		return
	}
	for _, neigh := range get_neighbours(position[0], position[1]) {
		if neigh[0] < 0 || neigh[1] < 0 || neigh[0] >= len(*m) || neigh[1] >= len((*m)[neigh[0]]) {
			continue
		}
		if (*m)[neigh[0]][neigh[1]] && (*visited)[neigh[0]][neigh[1]] - limit >= length {
			ends[neigh] = true
			continue
		}
		if (*m)[neigh[0]][neigh[1]] {
			continue
		}
		l, ok := previous[neigh]
		if ok && l <= length {
			continue
		}
		previous[neigh] = length
		find_shortcuts(m, visited, timer, limit, length, neigh, previous, ends)
	}
}

func count_shortcuts(picoseconds, limit int) int {
	result := 0
	m, start_pos, _ := read_file("INPUT")
	visited := make([][]int, len(m))
	for i, row := range m {
		visited[i] = make([]int, len(m[i]))
		for j := range row {
			visited[i][j] = Max
		}
	}
	find_shortest_path(&m, start_pos, 0, &visited)
	for i, row := range m {
		for j := range row {
			if m[i][j] {
				ends := make(map[[2]int]bool)
				find_shortcuts(&m, &visited, picoseconds, limit, visited[i][j], [2]int{i,j}, make(map[[2]int]int), ends)
				result += len(ends)
			}
		}
	}
	return result
}

func part_one() {
	var result int
	start := time.Now()
	result = count_shortcuts(2, 1)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	result = count_shortcuts(20, 50)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "20 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
