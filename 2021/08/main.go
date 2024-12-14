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

type Note struct {
	notes []string
	display []string
}

func read_file(file_path string) []Note {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var notes []Note
	for _, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		split := strings.Split(line, " | ")
		var note Note
		note.notes = strings.Split(split[0], " ")
		note.display = strings.Split(split[1], " ")
		notes = append(notes, note)
	}
	return notes
}

func part_one() {
	var result int
	start := time.Now()
	notes := read_file("INPUT")
	for _, note := range notes {
		for _, digit := range note.display {
			if len(digit) == 2 || len(digit) == 3 || len(digit) == 4 || len(digit) == 7 {
				result += 1
			}
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2021" + Reset + " day " + Green + "8 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
