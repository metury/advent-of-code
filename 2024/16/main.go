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

const (
	Wall = 1
	Space = 0
)

func read_file(file_path string) ([][]int8, [2]int, [2]int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var maze [][]int8
	var end [2]int
	var start [2]int
	for i, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		row := make([]int8, len(line))
		for j, c := range line {
			switch c {
				case '#':
					row[j] = Wall
				case '.':
					row[j] = Space
				case 'S':
					start = [2]int{i, j}
					row[j] = Space
				case 'E':
					end = [2]int{i, j}
					row[j] = Space
			}
		}
		maze = append(maze, row)
	}
	return maze, start, end
}

func find_path(maze [][]int8, current, end, orientation [2]int, visited map[[4]int]int, score int) int {
	if current == end {
		return score
	}
	res := int(^uint(0) >> 1)
	visited[[4]int{current[0], current[1], orientation[0], orientation[1]}] = score
	new_pos := [2]int{current[0] + orientation[0], current[1] + orientation[1]}
	length, ok := visited[[4]int{new_pos[0], new_pos[1], orientation[0], orientation[1]}]
	if (!ok || length > score + 1) && maze[new_pos[0]][new_pos[1]] == Space{
		res = find_path(maze, new_pos, end, orientation, visited, score + 1)
	}
	new_orientation := [2]int{-orientation[1], orientation[0]}
	new_pos = [2]int{current[0] + new_orientation[0], current[1] + new_orientation[1]}
	length, ok = visited[[4]int{new_pos[0], new_pos[1], new_orientation[0], new_orientation[1]}]
	if (!ok || length > score + 1001) && maze[new_pos[0]][new_pos[1]] == Space {
		new_res := find_path(maze, new_pos, end, new_orientation, visited, score + 1001)
		if res > new_res {
			res = new_res
		}

	}
	new_orientation = [2]int{orientation[1], -orientation[0]}
	new_pos = [2]int{current[0] + new_orientation[0], current[1] + new_orientation[1]}
	length, ok = visited[[4]int{new_pos[0], new_pos[1], new_orientation[0], new_orientation[1]}]
	if (!ok || length > score + 1001) && maze[new_pos[0]][new_pos[1]] == Space {
		new_res := find_path(maze, new_pos, end, new_orientation, visited, score + 1001)
		if res > new_res {
			res = new_res
		}

	}
	return res
}

func find_seats(maze [][]int8, current, end, orientation [2]int, visited map[[4]int]int, score int) (int, [][2]int) {
	if current == end {
		return score, [][2]int{end}
	}
	res := int(^uint(0) >> 1)
	var spots [][2]int
	visited[[4]int{current[0], current[1], orientation[0], orientation[1]}] = score
	new_pos := [2]int{current[0] + orientation[0], current[1] + orientation[1]}
	length, ok := visited[[4]int{new_pos[0], new_pos[1], orientation[0], orientation[1]}]
	if (!ok || length >= score + 1) && maze[new_pos[0]][new_pos[1]] == Space{
		res, spots = find_seats(maze, new_pos, end, orientation, visited, score + 1)
	}
	new_orientation := [2]int{-orientation[1], orientation[0]}
	new_pos = [2]int{current[0] + new_orientation[0], current[1] + new_orientation[1]}
	length, ok = visited[[4]int{new_pos[0], new_pos[1], new_orientation[0], new_orientation[1]}]
	if (!ok || length >= score + 1001) && maze[new_pos[0]][new_pos[1]] == Space {
		new_res, new_spots := find_seats(maze, new_pos, end, new_orientation, visited, score + 1001)
		if res > new_res {
			res = new_res
			spots = new_spots
		} else if res == new_res {
			spots = append(spots, new_spots[:]...)
		}

	}
	new_orientation = [2]int{orientation[1], -orientation[0]}
	new_pos = [2]int{current[0] + new_orientation[0], current[1] + new_orientation[1]}
	length, ok = visited[[4]int{new_pos[0], new_pos[1], new_orientation[0], new_orientation[1]}]
	if (!ok || length >= score + 1001) && maze[new_pos[0]][new_pos[1]] == Space {
		new_res, new_spots := find_seats(maze, new_pos, end, new_orientation, visited, score + 1001)
		if res > new_res {
			res = new_res
			spots = new_spots
		} else if res == new_res {
			spots = append(spots, new_spots[:]...)
		}

	}
	return res, append(spots, current)
}

func part_one() {
	var result int
	start := time.Now()
	maze, start_point, end_point := read_file("INPUT")
	result = find_path(maze, start_point, end_point, [2]int{0,1}, make(map[[4]int]int), 0)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	maze, start_point, end_point := read_file("INPUT")
	_, spots := find_seats(maze, start_point, end_point, [2]int{0,1}, make(map[[4]int]int), 0)
	seats := make(map[[2]int]bool)
	for _, s := range spots {
		seats[s] = true
	}
	result = len(seats)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "16 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
