package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"time"
)

func read_file(file_path string) []int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex := regexp.MustCompile("([0-9]+)")
	found := regex.FindAllStringSubmatch(string(content), -1)
	array := make([]int, len(found))
	for i, f := range found {
		array[i], _ = strconv.Atoi(f[1])
	}
	return array
}

func slide(start []int) func(int) bool {
	window := start
	sum := 0
	for _, ele := range window {
		sum += ele
	}
	return func(ele int) bool {
		new_sum := sum - window[0] + ele
		window = append(window[1:], ele)
		res := new_sum > sum
		sum = new_sum
		return res
	}
}

func part1() {
	var result int
	start := time.Now()
	array := read_file("INPUT")
	s := slide(array[:1])
	for i := 1; i < len(array); i++ {
		if s(array[i]) {
			result += 1
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	array := read_file("INPUT")
	s := slide(array[:3])
	for i := 3; i < len(array); i++ {
		if s(array[i]) {
			result += 1
		}
	}
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2021 day 1 - Sonar Sweep")
	part1()
	part2()
}
