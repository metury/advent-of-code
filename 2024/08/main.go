package main

import (
	"fmt"
	"log"
	"os"
	"strings"
	"time"
)

type Position [2]int

func read_file(file_path string) (map[string][]Position, int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	m := make(map[string][]Position)
	size := 0
	for i, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		size += 1
		chars := strings.Split(line, "")
		for j, c := range chars {
			if c != "." {
				val, ok := m[c]
				if ok {
					m[c] = append(val, [2]int{i, j})
				} else {
					m[c] = []Position{[2]int{i,j}}
				}
			}
		}
	}
	return m, size
}

func is_in_map(pos Position, size int) bool {
	return pos[0] >= 0 && pos[1] >= 0 && pos[0] < size && pos[1] < size
}

func add_antinode(nodes *map[[2]int]bool, antenna, diff Position, op func(int, int) int, size int, first_only bool) {
	antinode := [2]int{op(antenna[0], diff[0]), op(antenna[1], diff[1])}
	for is_in_map(antinode, size) {
		(*nodes)[antinode] = true
		if first_only {
			return
		}
		antinode = [2]int{op(antinode[0], diff[0]), op(antinode[1], diff[1])}
	}
}

func add_nodes(nodes *map[[2]int]bool, first_antenna, second_antenna Position, size int, first_only bool) {
	diff := [2]int{first_antenna[0] - second_antenna[0], first_antenna[1] - second_antenna[1]}
	if !first_only {
		(*nodes)[first_antenna] = true
		(*nodes)[second_antenna] = true
	}
	add := func(x int, y int) int {return x + y;}
	sub := func(x int, y int) int {return x - y;}
	add_antinode(nodes, first_antenna, diff, add, size, first_only)
	add_antinode(nodes, second_antenna, diff, sub, size, first_only)
}

func part1() {
	var result int
	start := time.Now()
	m, size := read_file("INPUT")
	nodes := make(map[[2]int]bool)
	for _, antennas := range m {
		for i := 0; i < len(antennas) -1; i++ {
			for j := i + 1; j < len(antennas); j++{
				add_nodes(&nodes, antennas[i], antennas[j], size, true)
			}
		}
	}
	result = len(nodes)
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	m, size := read_file("INPUT")
	nodes := make(map[[2]int]bool)
	for _, antennas := range m {
		for i := 0; i < len(antennas) -1; i++ {
			for j := i + 1; j < len(antennas); j++{
				add_nodes(&nodes, antennas[i], antennas[j], size, false)
			}
		}
	}
	result = len(nodes)
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 8 - Resonant Collinearity")
	part1()
	part2()
}
