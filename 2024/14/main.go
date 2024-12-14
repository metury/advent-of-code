package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
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

const (
	Width  = 101
	Height = 103
)

type Robot struct {
	position [2]int
	velocity [2]int
}

func mod(nr, mod int) int {
	return (nr%mod + mod) % mod
}

func (r *Robot) move(iter int) {
	r.position[0] += r.velocity[0] * iter
	r.position[1] += r.velocity[1] * iter
	r.position[0] = mod(r.position[0], Width)
	r.position[1] = mod(r.position[1], Height)
}

func read_file(file_path string) []Robot {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var robots []Robot
	regex := regexp.MustCompile("p=([\\-0-9]+),([\\-0-9]+) v=([\\-0-9]+),([\\-0-9]+)")
	found := regex.FindAllStringSubmatch(string(content), -1)
	for _, f := range found {
		p_x, _ := strconv.Atoi(f[1])
		p_y, _ := strconv.Atoi(f[2])
		v_x, _ := strconv.Atoi(f[3])
		v_y, _ := strconv.Atoi(f[4])
		robot := Robot{
			position: [2]int{p_x, p_y},
			velocity: [2]int{v_x, v_y},
		}
		robots = append(robots, robot)
	}
	return robots
}

func is_tree(bitmap [Width][Height]bool) bool {
	for i := 0; i < Height; i++ {
		for j := 0; j < Width; j++ {
			k := j
			for ; k < Width && bitmap[k][i]; k++ {
			}
			if k-j > 30 {
				return true
			}
		}
	}
	return false
}

func part_one() {
	var result int
	start := time.Now()
	robots := read_file("INPUT")
	var quadrants [4]int
	for i := range robots {
		robots[i].move(100)
		if robots[i].position[0] < Width/2 {
			if robots[i].position[1] < Height/2 {
				quadrants[0] += 1
			} else if robots[i].position[1] > Height/2 {
				quadrants[1] += 1
			}
		} else if robots[i].position[0] > Width/2 {
			if robots[i].position[1] < Height/2 {
				quadrants[2] += 1
			} else if robots[i].position[1] > Height/2 {
				quadrants[3] += 1
			}
		}
	}
	result = quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3]
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	robots := read_file("INPUT")
	for t := 1; ; t++ {
		var bitmap [Width][Height]bool
		for i := range robots {
			robots[i].move(1)
			bitmap[robots[i].position[0]][robots[i].position[1]] = true
		}
		if is_tree(bitmap) {
			result = t
			break
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "14 - Restroom Redoubt" + Reset)
	part_one()
	part_two()
}
