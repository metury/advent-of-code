package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strings"
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

// Create a table from the input.
func tabelize(text *string) [][]string {
	lines := strings.Split(*text, "\n")
	var table [][]string
	for _, line := range lines {
		if len(line) > 0 {
			table = append(table, strings.Split(line, ""))
		}
	}
	return table
}

// Transpose the text as a matrix.
func transpose(text *string) string {
	var builder strings.Builder
	table := tabelize(text)
	for i := range table[0] {
		for j := range table {
			builder.WriteString(string(table[j][i]))
		}
		builder.WriteString("\n")
	}
	return builder.String()
}

// Create all diagonals of the text.
func diag(text *string) string {
	var builder strings.Builder
	table := tabelize(text)
	for i := len(table[0]) - 1; i >= 0; i-- {
		for j := 0; j+i < len(table[0]); j++ {
			builder.WriteString(table[j][j+i])
		}
		builder.WriteString("\n")
	}
	for i := len(table) - 1; i > 0; i-- {
		for j := 0; j+i < len(table); j++ {
			builder.WriteString(table[i+j][j])
		}
		builder.WriteString("\n")
	}
	return builder.String()
}

// Swap the order of lines.
func swap(text *string) string {
	var builder strings.Builder
	lines := strings.Split(*text, "\n")
	for i := len(lines) - 1; i >= 0; i-- {
		builder.WriteString(lines[i])
		builder.WriteString("\n")
	}
	return builder.String()
}

// Replace all S by SS and X by XX.
func replace(text *string) string {
	s := regexp.MustCompile("S")
	x := regexp.MustCompile("X")
	return x.ReplaceAllString(s.ReplaceAllString(*text, "SS"), "XX")
}

func count_xmas(text *string) int {
	regex := regexp.MustCompile("XMAS|SAMX")
	found := regex.FindAllString(*text, -1)
	if found == nil {
		return 0
	}
	return len(found)
}

func count_x_mas(table [][]string) int {
	res := 0
	for i := 1; i < len(table)-1; i++ {
		for j := 1; j < len(table[i])-1; j++ {
			if table[i][j] == "A" {
				first_word := table[i-1][j-1] + table[i][j] + table[i+1][j+1]
				second_word := table[i-1][j+1] + table[i][j] + table[i+1][j-1]
				regex := regexp.MustCompile("MAS|SAM")
				if regex.FindAllString(first_word, 1) != nil && regex.FindAllString(second_word, 1) != nil {
					res += 1
				}
			}
		}
	}
	return res
}

func part_one() {
	var result int
	var content = read_file("INPUT")
	start := time.Now()
	var builder strings.Builder
	swapped := swap(&content)
	builder.WriteString(content)
	builder.WriteString("\n")
	builder.WriteString(transpose(&content))
	builder.WriteString("\n")
	builder.WriteString(diag(&content))
	builder.WriteString("\n")
	builder.WriteString(diag(&swapped))
	text := builder.String()
	text = replace(&text)
	result = count_xmas(&text)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	var content = read_file("INPUT")
	start := time.Now()
	table := tabelize(&content)
	result = count_x_mas(table)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "4 - Ceres Search" + Reset)
	part_one()
	part_two()
}
