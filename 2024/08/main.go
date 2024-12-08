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

func part1() {
	var result int
	start := time.Now()
	m, size := read_file("INPUT")
	nodes := make(map[[2]int]bool)
	for _, antenas := range m {
		for i := 0; i < len(antenas) -1; i++ {
			for j := i + 1; j < len(antenas); j++{
				diff := [2]int{antenas[i][0] - antenas[j][0], antenas[i][1] - antenas[j][1]}
				first_antinode := [2]int{antenas[i][0] + diff[0], antenas[i][1] + diff[1]}
				second_antinode := [2]int{antenas[j][0] - diff[0], antenas[j][1] - diff[1]}
				if is_in_map(first_antinode, size) {
					nodes[first_antinode] = true
				}
				if is_in_map(second_antinode, size) {
					nodes[second_antinode] = true
				}
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
	for _, antenas := range m {
		for i := 0; i < len(antenas) -1; i++ {
			for j := i + 1; j < len(antenas); j++{
				diff := [2]int{antenas[i][0] - antenas[j][0], antenas[i][1] - antenas[j][1]}
				nodes[antenas[i]] = true
				nodes[antenas[j]] = true
				first_antinode := [2]int{antenas[i][0] + diff[0], antenas[i][1] + diff[1]}
				for is_in_map(first_antinode, size) {
					nodes[first_antinode] = true
					first_antinode = [2]int{first_antinode[0] + diff[0], first_antinode[1] + diff[1]}
				}
				second_antinode := [2]int{antenas[j][0] - diff[0], antenas[j][1] - diff[1]}
				for is_in_map(second_antinode, size) {
					nodes[second_antinode] = true
					second_antinode = [2]int{second_antinode[0] - diff[0], second_antinode[1] - diff[1]}
				}
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
