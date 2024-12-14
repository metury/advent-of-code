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

func read_file(file_path string) ([]int, []int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex := regexp.MustCompile("([0-9]+)\\s+([0-9]+)")
	found := regex.FindAllStringSubmatch(string(content), -1)
	var first_column []int
	var second_column []int
	for _, f := range found {
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
	for _, val := range *list {
		if i, ok := my_map[val]; ok {
			my_map[val] = i + 1
		} else {
			my_map[val] = 1
		}
	}
	return my_map
}

func part_one() {
	var result int
	start := time.Now()
	first_column, second_column := read_file("INPUT")
	for i := range first_column {
		difference := math.Abs(float64(first_column[i] - second_column[i]))
		result += int(difference)
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	first_column, second_column := read_file("INPUT")
	second_map := count(&second_column)
	for _, key := range first_column {
		i, ok := second_map[key]
		if ok {
			result = result + key*i
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "1 - Historian Hysteria" + Reset)
	part_one()
	part_two()
}
