package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"sort"
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
	regex := regexp.MustCompile("[0-9]+")
	found := regex.FindAllString(string(content), -1)
	var horizontal_positions []int
	for _, f := range found {
		nr, _ := strconv.Atoi(f)
		horizontal_positions = append(horizontal_positions, nr)
	}
	return horizontal_positions
}

func optimize_distance(numbers []int, op func(int, int) int) int {
	sort.Ints(numbers)
	result := op(numbers[0], numbers[len(numbers)-1]) * len(numbers)
	for x := numbers[0]; x < numbers[len(numbers)-1]; x++ {
		sum := 0
		for _, pos := range numbers {
			sum += op(x, pos)
		}
		if sum < result {
			result = sum
		}
	}
	return result
}

func part_one() {
	var result int
	start := time.Now()
	horizontal_positions := read_file("INPUT")
	dist := func(x, y int) int { return int(math.Abs(float64(x - y))) }
	result = optimize_distance(horizontal_positions, dist)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	horizontal_positions := read_file("INPUT")
	scaled_dist := func(x, y int) int {
		diff := int(math.Abs(float64(x - y)))
		return (diff * (diff + 1)) / 2
	}
	result = optimize_distance(horizontal_positions, scaled_dist)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2021" + Reset + " day " + Green + "7 - The Treachery of Whales" + Reset)
	part_one()
	part_two()
}
