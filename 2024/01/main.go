package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"sort"
	"math"
)

func read_file(file_path string) ([]int, []int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex, _ := regexp.Compile("([0-9]+)\\s+([0-9]+)")
	found := (regex.FindAllStringSubmatch(string(content), -1))
	var first_column []int
	var second_column []int
	for _, f := range(found) {
		nr1, _ := strconv.Atoi(f[1])
		nr2, _ := strconv.Atoi(f[2])
		first_column = append(first_column, nr1)
		second_column = append(second_column, nr2)
	}
	sort.Ints(first_column)
	sort.Ints(second_column)
	return first_column, second_column
}

func count(list *[]int) map[int]int {
	my_map := make(map[int]int)
	for _,val := range *list {
		i, ok := my_map[val]
		if ok {
			my_map[val] = i + 1
		} else {
			my_map[val] = 1
		}
	}
	return my_map
}

func part1() {
	var result int = 0
	read_file("INPUT")
	first_column, second_column := read_file("INPUT")
	for i,_ := range first_column {
		difference := math.Abs(float64(first_column[i] - second_column[i]))
		result += int(difference)
	}
	fmt.Println("Part 1: ", result)
}

func part2() {
	var result int = 0
	first_column, second_column := read_file("INPUT")
	second_map := count(&second_column)
	for _, key := range first_column {
		i, ok := second_map[key]
		if ok {
			result = result + key * i
		}
	}
	fmt.Println("Part 2: ", result)
}

func main() {
	fmt.Println("Year 2024 day 1 - Historian Hysteria")
	part1()
	part2()
}