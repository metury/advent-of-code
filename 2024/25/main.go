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

func print_result(dur time.Duration, part int, result string) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

func read_file(file_path string) ([][5]int, [][5]int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}

	lines := strings.Split(string(content), "\n")
	var locks [][5]int
	var keys [][5]int
	var buffer [5]int
	key := false
	index := 0
	for _, line := range lines {
		if len(line) == 0 {
			buffer[0], buffer[1], buffer[2], buffer[3], buffer[4] = buffer[0]-1, buffer[1]-1, buffer[2]-1, buffer[3]-1, buffer[4]-1
			if key {
				keys = append(keys, buffer)
			} else {
				locks = append(locks, buffer)
			}
			buffer[0], buffer[1], buffer[2], buffer[3], buffer[4] = 0, 0, 0, 0, 0
			continue
		}
		if index%7 == 0 {
			if line[0] == '#' {
				key = false
			} else {
				key = true
			}
		}
		for i, chr := range line {
			if chr == '#' {
				buffer[i] += 1
			}
		}
		index++
	}
	return keys, locks
}

func try_all(keys [][5]int, locks [][5]int) int {
	sum := 0
	for _, key := range keys {
	outer:
		for _, lock := range locks {
			for i := 0; i < 5; i++ {
				if key[i]+lock[i] >= 6 {
					continue outer
				}
			}
			sum += 1
		}
	}
	return sum
}

func part_one() {
	var result int
	start := time.Now()
	keys, locks := read_file("INPUT")
	result = try_all(keys, locks)
	end := time.Now()
	print_result(end.Sub(start), 1, fmt.Sprint(result))
}

func part_two() {
	var result string
	start := time.Now()
	result = "Deliver the Chronicle"
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "25 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
