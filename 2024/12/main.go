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
	HORIZONTAL = 0
	VERTICAL = 1
	UP = 2
	DOWN = 3
)

type Fence struct {
	position [2]int
	orientation int
	side int
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

func check_similarity(considered *map[[2]int]bool, array *[]int, current, fence Fence, index, orientation int) {
	_, ok := (*considered)[[2]int{current.position[index], current.side}]
	if current.position[index] == fence.position[index] && !ok && current.orientation == orientation && fence.orientation == orientation && current.side == fence.side {
		*array = append(*array, fence.position[(index + 1) % 2])
	}
}

func count_sides(array []int) int {
	sum := 0
	sort.Ints(array)
	for j := 1; j < len(array); j++ {
		if int(math.Abs(float64(array[j - 1] - array[j]))) > 1 {
			sum += 1
		}
	}
	if len(array) > 0 {
		sum += 1
	}
	return sum
}

func filter_count(fences map[Fence]bool) int {
	considered_vertical := make(map[[2]int]bool)
	considered_horizontal := make(map[[2]int]bool)
	sum := 0
	for current := range fences {
		vertical := make([]int, 0)
		horizontal := make([]int, 0)
		for fence := range fences {
			check_similarity(&considered_vertical, &vertical, current, fence, 0, VERTICAL)
			check_similarity(&considered_horizontal, &horizontal, current, fence, 1, HORIZONTAL)
		}
		if current.orientation == VERTICAL {
			considered_vertical[[2]int{current.position[0], current.side}] = true
		} else {
			considered_horizontal[[2]int{current.position[1], current.side}] = true
		}
		sum += count_sides(horizontal) + count_sides(vertical)
	}
	return sum
}

func insert_fence(fences *map[Fence]bool, current, neigh [2]int) {
	if  neigh[0] == current[0] {
		if neigh[1] < current[1] {
			(*fences)[Fence{position: neigh, orientation: HORIZONTAL, side: UP}] = true
		} else {
			(*fences)[Fence{position: neigh, orientation: HORIZONTAL, side: DOWN}] = true
		}
	} else {
		if neigh[0] < current[0] {
			(*fences)[Fence{position: neigh, orientation: VERTICAL, side: UP}] = true
		} else {
			(*fences)[Fence{position: neigh, orientation: VERTICAL, side: DOWN}] = true
		}
	}
}

func bucket(garden *[][]string, visited_map *map[[2]int]bool, i, j int) (map[Fence]bool, map[[2]int]bool) {
	var queue [][2]int
	queue = append(queue, [2]int{i,j})
	fences := make(map[Fence]bool)
	inner := make(map[[2]int]bool)

	for len(queue) > 0 {
		current := queue[0]
		inner[current] = true
		queue = queue[1:]
		neighbours := [4][2]int {
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

func count_fence(garden *[][]string, f func(map[Fence]bool) int) int {
	sum := 0
	visited := make(map[[2]int]bool)
	for i, row := range *garden {
		for j := range row {
			_, ok := visited[[2]int{i,j}]
			if ok {
				continue
			}
			fences, inner := bucket(garden, &visited, i, j)
			sum += len(inner) * f(fences)
		}
	}
	return sum
}

func part1() {
	var result int
	start := time.Now()
	garden := read_file("INPUT")
	my_len := func(m map[Fence]bool) int { return len(m); }
	result = count_fence(&garden, my_len)
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	garden := read_file("INPUT")
	result = count_fence(&garden, filter_count)
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 12 - Garden Groups")
	part1()
	part2()
}
