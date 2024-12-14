package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
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

func read_file(file_path string) [][]int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	lines := strings.Split(string(content), "\n")
	var m [][]int
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		tiles := strings.Split(line, "")
		row := make([]int, len(tiles))
		for i, tile := range tiles {
			row[i], _ = strconv.Atoi(tile)
		}
		m = append(m, row)
	}
	return m
}

func get_neighbours(x, y int) [4][2]int {
	return [4][2]int{
		{x + 1, y},
		{x - 1, y},
		{x, y + 1},
		{x, y - 1},
	}
}

func in_map(m *[][]int, x, y int) bool {
	return x >= 0 && y >= 0 && x < len(*m) && y < len((*m)[0])
}

func find_trails(m *[][]int, x, y int) map[[2]int]bool {
	if (*m)[x][y] == 9 {
		res := make(map[[2]int]bool)
		res[[2]int{x, y}] = true
		return res
	}
	res := make(map[[2]int]bool)
	neighbours := get_neighbours(x, y)
	for _, neigh := range neighbours {
		if in_map(m, neigh[0], neigh[1]) && (*m)[neigh[0]][neigh[1]] == (*m)[x][y]+1 {
			for key, val := range find_trails(m, neigh[0], neigh[1]) {
				res[key] = val
			}
		}
	}
	return res
}

func find_distinct(m *[][]int, x, y int) int {
	if (*m)[x][y] == 9 {
		return 1
	}
	res := 0
	neighbours := get_neighbours(x, y)
	for _, neigh := range neighbours {
		if in_map(m, neigh[0], neigh[1]) && (*m)[neigh[0]][neigh[1]] == (*m)[x][y]+1 {
			res += find_distinct(m, neigh[0], neigh[1])
		}
	}
	return res
}

func part_one() {
	var result int
	start := time.Now()
	m := read_file("INPUT")
	for i, row := range m {
		for j := range row {
			if m[i][j] == 0 {
				result += len(find_trails(&m, i, j))
			}
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	m := read_file("INPUT")
	for i, row := range m {
		for j := range row {
			if m[i][j] == 0 {
				result += find_distinct(&m, i, j)
			}
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "10 - Hoof It" + Reset)
	part_one()
	part_two()
}
