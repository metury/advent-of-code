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

func read_file(file_path string) []int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var stones []int
	regex := regexp.MustCompile("([0-9]+)")
	found := regex.FindAllString(string(content), -1)
	for _, f := range found {
		nr, _ := strconv.Atoi(f)
		stones = append(stones, nr)
	}
	return stones
}

func split_size(stone int) (int, int, bool) {
	str := strconv.Itoa(stone)
	if len(str)%2 == 0 {
		prefix, _ := strconv.Atoi(str[:len(str)/2])
		postfix, _ := strconv.Atoi(str[len(str)/2:])
		return prefix, postfix, true
	}
	return -1, -1, false
}

func add_to_map(stones *map[int]int, key, val int) {
	_, ok := (*stones)[key]
	if ok {
		(*stones)[key] += val
	} else {
		(*stones)[key] = val
	}
}

func blink182(stones map[int]int, iter int) int {
	for i := 0; i < iter; i++ {
		new_stones := make(map[int]int)
		for key, val := range stones {
			if val > 0 {
				prefix, postfix, even := split_size(key)
				if key == 0 {
					add_to_map(&new_stones, 1, val)
				} else if even {
					add_to_map(&new_stones, prefix, val)
					add_to_map(&new_stones, postfix, val)
				} else {
					add_to_map(&new_stones, 2024*key, val)
				}
			}
		}
		stones = new_stones
	}
	sum := 0
	for _, val := range stones {
		sum += val
	}
	return sum
}

func count_all(iter int) int {
	stones := read_file("INPUT")
	m := make(map[int]int)
	for _, stone := range stones {
		add_to_map(&m, stone, 1)
	}
	return blink182(m, iter)
}

func part_one() {
	var result int
	start := time.Now()
	result = count_all(25)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	result = count_all(75)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "11 - Plutonian Pebbles" + Reset)
	part_one()
	part_two()
}
