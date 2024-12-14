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

func read_file(file_path string) [9]int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex := regexp.MustCompile("[0-9]+")
	found := regex.FindAllString(string(content), -1)
	var lanternfish [9]int
	for _, f := range found {
		index, _ := strconv.Atoi(f)
		lanternfish[index] += 1
	}
	return lanternfish
}

func simulate_life(lanternfish [9]int, days int) int {
	for i := 0; i < days; i++ {
		first := lanternfish[0]
		for j := 1; j < len(lanternfish); j++ {
			lanternfish[j-1] =lanternfish[j]
		}
		lanternfish[8] = first
		lanternfish[6] += first
	}
	sum := 0
	for i := 0; i < len(lanternfish); i++ {
		sum += lanternfish[i]
	}
	return sum
}

func part_one() {
	var result int
	start := time.Now()
	lanternfish := read_file("INPUT")
	result = simulate_life(lanternfish, 80)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	lanternfish := read_file("INPUT")
	result = simulate_life(lanternfish, 256)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2021" + Reset + " day " + Green + "6 - Lanternfish" + Reset)
	part_one()
	part_two()
}
