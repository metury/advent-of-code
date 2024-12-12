package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"sort"
	"strings"
	"time"
)

const (
	UPPER = 0
	LOWER = 2
	LEFT  = 1
	RIGHT = 3
)

type Fence struct {
	position [2]int
	side     int
}

func read_file(file_path string) [][]string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var m [][]string
	lines := strings.Split(string(content), "\n")
	for _, line := range lines {
		if len(line) == 0 {
			continue
		}
		chars := strings.Split(line, "")
		m = append(m, chars)

	}
	return m
}

func count_sides(fences map[Fence]bool) int {
	considered := make(map[[2]int]bool)
	sum := 0
	for current := range fences {
		i := current.side % 2
		_, used := considered[[2]int{current.position[i], current.side}]
		if used {
			continue
		}
		array := make([]int, 0)
		for fence := range fences {
			if current.side == fence.side && current.position[i] == fence.position[i] {
				array = append(array, fence.position[(i+1)%2])
			}
		}
		considered[[2]int{current.position[i], current.side}] = true
		sort.Ints(array)
		for j := 1; j < len(array); j++ {
			if int(math.Abs(float64(array[j-1]-array[j]))) > 1 {
				sum += 1
			}
		}
		if len(array) > 0 {
			sum += 1
		}
	}
	return sum
}

func insert_fence(fences *map[Fence]bool, current, neigh [2]int) {
	if neigh[0] == current[0] {
		if neigh[1] < current[1] {
			(*fences)[Fence{position: neigh, side: RIGHT}] = true
		} else {
			(*fences)[Fence{position: neigh, side: LEFT}] = true
		}
	} else {
		if neigh[0] < current[0] {
			(*fences)[Fence{position: neigh, side: UPPER}] = true
		} else {
			(*fences)[Fence{position: neigh, side: LOWER}] = true
		}
	}
}

func flood_fill(garden *[][]string, visited_map *map[[2]int]bool, i, j int) (map[Fence]bool, map[[2]int]bool) {
	queue := append(make([][2]int, 0), [2]int{i, j})
	fences := make(map[Fence]bool)
	inner := make(map[[2]int]bool)

	for len(queue) > 0 {
		current := queue[0]
		inner[current] = true
		queue = queue[1:]
		neighbours := [4][2]int{
			{current[0] - 1, current[1]},
			{current[0] + 1, current[1]},
			{current[0], current[1] + 1},
			{current[0], current[1] - 1},
		}
		for _, neigh := range neighbours {
			_, visited := inner[neigh]
			if visited {
				continue
			}
			if neigh[0] < 0 || neigh[1] < 0 || neigh[0] >= len(*garden) || neigh[1] >= len((*garden)[0]) {
				insert_fence(&fences, current, neigh)
				continue
			}
			if (*garden)[neigh[0]][neigh[1]] != (*garden)[i][j] {
				insert_fence(&fences, current, neigh)
				continue
			}
			queue = append(queue, neigh)
			inner[neigh] = true
			(*visited_map)[neigh] = true
		}
	}
	return fences, inner
}

func count_fences(garden *[][]string, f func(map[Fence]bool) int) int {
	sum := 0
	visited := make(map[[2]int]bool)
	for i, row := range *garden {
		for j := range row {
			_, ok := visited[[2]int{i, j}]
			if ok {
				continue
			}
			fences, inner := flood_fill(garden, &visited, i, j)
			sum += len(inner) * f(fences)
		}
	}
	return sum
}

func part_one() {
	var result int
	start := time.Now()
	garden := read_file("INPUT")
	my_len := func(m map[Fence]bool) int { return len(m) }
	result = count_fences(&garden, my_len)
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	start := time.Now()
	garden := read_file("INPUT")
	result = count_fences(&garden, count_sides)
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 12 - Garden Groups")
	part_one()
	part_two()
}
