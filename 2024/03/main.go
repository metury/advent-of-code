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

func read_file(file_path string) string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}

func part_one() {
	var result int
	var content = read_file("INPUT")
	start := time.Now()
	regex := regexp.MustCompile("mul\\(([0-9]{1,3}),([0-9]{1,3})\\)")
	found := regex.FindAllStringSubmatch(content, -1)
	for _, f := range found {
		nr1, _ := strconv.Atoi(f[1])
		nr2, _ := strconv.Atoi(f[2])
		result += nr1 * nr2
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	content := read_file("INPUT")
	start := time.Now()
	regex := regexp.MustCompile("do\\(\\)|don't\\(\\)|mul\\(([0-9]{1,3}),([0-9]{1,3})\\)")
	found := regex.FindAllStringSubmatch(content, -1)
	enabled := true
	for _, f := range found {
		switch f[0] {
		case "do()":
			enabled = true
		case "don't()":
			enabled = false
		default:
			if enabled {
				nr1, _ := strconv.Atoi(f[1])
				nr2, _ := strconv.Atoi(f[2])
				result += nr1 * nr2
			}
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "3 - Mull It Over" + Reset)
	part_one()
	part_two()
}
