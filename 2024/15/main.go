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
	Space = 0
	Wall = 1
	Box = 2
	LeftBox = 3
	RightBox = 4
)

func read_file(file_path string) ([][]int8, [2]int, [][2]int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var m [][]int8
	var position [2]int
	var moves [][2]int
	for i, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		if line[0] == '#' {
			row := make([]int8, len(line))
			for j, char := range strings.Split(line, "") {
				switch char {
					case "#":
						row[j] = Wall
					case "O":
						row[j] = Box
					case "@":
						row[j] = Space
						position = [2]int{i,j}
					case ".":
						row[j] = Space
				}
			}
			m = append(m, row)
		} else {
			for _, char := range strings.Split(line, "") {
				switch char {
					case ">":
						moves = append(moves, [2]int{0,1})
					case "<":
						moves = append(moves, [2]int{0,-1})
					case "^":
						moves = append(moves, [2]int{-1,0})
					case "v":
						moves = append(moves, [2]int{1,0})
				}
			}
		}
	}
	return m, position, moves
}

func wider_map(m [][]int8) [][]int8{
	wider := make([][]int8, len(m))
	for i, row := range m {
		var new_row []int8
		for j := range row {
			switch row[j] {
				case Space:
					new_row = append(new_row, Space, Space)
				case Wall:
					new_row = append(new_row, Wall, Wall)
				case Box:
					new_row = append(new_row, LeftBox, RightBox)
			}
		}
		wider[i] = new_row
	}
	return wider
}

func up_down_move(m *[][]int8, box [2]int, move [2]int) {
	if (*m)[box[0]][box[1]] == LeftBox {
		if (*m)[box[0] + move[0]][box[1]] == LeftBox || (*m)[box[0] + move[0]][box[1]] == RightBox{
			up_down_move(m, [2]int{box[0] + move[0], box[1]}, move)
		}
		if (*m)[box[0] + move[0]][box[1] + 1] == LeftBox {
			up_down_move(m, [2]int{box[0] + move[0], box[1] + 1}, move)
		}
		(*m)[box[0]][box[1]] = Space
		(*m)[box[0]][box[1] + 1] = Space
		(*m)[box[0] + move[0]][box[1]] = LeftBox
		(*m)[box[0] + move[0]][box[1] + 1] = RightBox
	} else if (*m)[box[0]][box[1]] == RightBox {
		up_down_move(m, [2]int{box[0], box[1] - 1}, move)
	}
}

func up_down_check(m *[][]int8, box [2]int, move [2]int) bool {
	if box[0] + move[0] < 0 || box[0] + move[0] >= len(*m) {
		return false
	}
	plausible := true
	if (*m)[box[0]][box[1]] == LeftBox {
		if (*m)[box[0] + move[0]][box[1]] == LeftBox || (*m)[box[0] + move[0]][box[1]] == RightBox {
			plausible = plausible && up_down_check(m, [2]int{box[0] + move[0], box[1]}, move)
		}
		if (*m)[box[0] + move[0]][box[1] + 1] == LeftBox {
			plausible = plausible && up_down_check(m, [2]int{box[0] + move[0], box[1] + 1}, move)
		}
		if (*m)[box[0] + move[0]][box[1]] == Space && (*m)[box[0] + move[0]][box[1] + 1] == Space {
			plausible = plausible && true
		}
		if (*m)[box[0] + move[0]][box[1]] == Wall || (*m)[box[0] + move[0]][box[1] + 1] == Wall {
			plausible = false
		}
	} else if (*m)[box[0]][box[1]] == RightBox {
		plausible = plausible && up_down_check(m, [2]int{box[0], box[1] - 1}, move)
	}
	return plausible
}

func move_larger_box(m *[][]int8, box [2]int, move [2]int) bool {
	if move[0] == 0 {
		i := 0
		for ;box[1] + i * move[1] < len((*m)[box[0]]) && ((*m)[box[0]][box[1] + i * move[1]] == LeftBox || (*m)[box[0]][box[1] + i * move[1]] == RightBox); i += 1 {
		}
		if box[1] + i * move[1] < len((*m)[box[0]]) && (*m)[box[0]][box[1] + i * move[1]] == Space {
			(*m)[box[0]][box[1]] = Space
			(*m)[box[0]][box[1] + i* move[1]] = (*m)[box[0]][box[1] + (i - 1) * move[1]]
			for j := 1; j < i; j++ {
				if (*m)[box[0]][box[1] + j * move[1]] == RightBox {
					(*m)[box[0]][box[1] + j * move[1]] = LeftBox
				} else {
					(*m)[box[0]][box[1] + j * move[1]] = RightBox
				}
			}
			return true
		}
	} else {
		if up_down_check(m, box, move) {
			up_down_move(m, box, move)
			return true
		}
	}
	return false
}

func move_box(m *[][]int8, box [2]int, move [2]int) bool {
	i := 0
	for ; box[0] + i * move[0] < len(*m) && box[1] + i * move[1] < len((*m)[box[0]]) && (*m)[box[0] + i * move[0]][box[1] + i * move[1]] == Box; i++ {
	}
	if box[0] + i * move[0] < len(*m) && box[1] + i * move[1] < len((*m)[box[0]]) && (*m)[box[0] + i * move[0]][box[1] + i * move[1]] == Space {
		(*m)[box[0]][box[1]] = Space
		(*m)[box[0] + i * move[0]][box[1] + i * move[1]] = Box
		return true
	}
	return false
}

func single_step(m *[][]int8, position *[2]int, move [2]int) {
	new_position := [2]int{
		(*position)[0] + move[0],
		(*position)[1] + move[1],
	}
	switch (*m)[new_position[0]][new_position[1]] {
		case Space:
			*position = new_position
		case Wall:
			return
		case Box:
			if move_box(m, new_position, move) {
				*position = new_position
			}
		case LeftBox:
			fallthrough
		case RightBox:
			if move_larger_box(m, new_position, move) {
				*position = new_position
			}
	}
}

func simulate(m *[][]int8, position *[2]int, moves [][2]int) int {
	for _, move := range moves {
		single_step(m, position, move)
	}
	sum := 0
	for i, row := range (*m) {
		for j := range row {
			if row[j] == Box || row[j] == LeftBox {
				sum += i*100 + j
			}
		}
	}
	return sum
}

func part_one() {
	var result int
	start := time.Now()
	m, position, moves := read_file("INPUT")
	result = simulate(&m, &position, moves)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	m, position, moves := read_file("INPUT")
	wider := wider_map(m)
	position[1] *= 2
	result = simulate(&wider, &(position), moves)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "15 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
