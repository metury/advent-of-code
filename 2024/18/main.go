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
	Blue   = "[1;34m"
	Yellow = "[1;33m"
	Green  = "[1;32m"
	Reset  = "[0m"
)

func print_result(dur time.Duration, part int, result string) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

const (
	Size = 71
	Rep  = 1024
)

var Start = [2]int{0, 0}
var End = [2]int{Size - 1, Size - 1}

func read_file(file_path string) [][2]int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var numbers [][2]int
	regex := regexp.MustCompile("([0-9]+),([0-9]+)")
	for _, f := range regex.FindAllStringSubmatch(string(content), -1) {
		nr1, _ := strconv.Atoi(f[1])
		nr2, _ := strconv.Atoi(f[2])
		numbers = append(numbers, [2]int{nr1, nr2})
	}
	return numbers
}

func find_path(m [Size][Size]bool, current, previous, end [2]int, visited map[[2]int][3]int, length int) {
	if current[0] < 0 || current[1] < 0 || current[0] >= Size || current[1] >= Size || m[current[0]][current[1]] {
		return
	}
	prev, ok := visited[current]
	l := prev[0]
	if !ok || l > length {
		visited[current] = [3]int{length, previous[0], previous[1]}
		if current[0] == end[0] && current[1] == end[1] {
			return
		}
		find_path(m, [2]int{current[0] + 1, current[1]}, current, end, visited, length+1)
		find_path(m, [2]int{current[0] - 1, current[1]}, current, end, visited, length+1)
		find_path(m, [2]int{current[0], current[1] + 1}, current, end, visited, length+1)
		find_path(m, [2]int{current[0], current[1] - 1}, current, end, visited, length+1)
	}
}

func recreate_path(visited map[[2]int][3]int) map[[2]int]bool {
	m := make(map[[2]int]bool)
	m[End] = true
	for current := End; current != Start; {
		prev, _ := visited[current]
		current = [2]int{prev[1], prev[2]}
		m[current] = true
	}
	return m
}

func part_one() {
	var result int
	start := time.Now()
	bytes := read_file("INPUT")
	var m [Size][Size]bool
	for i := 0; i < Rep; i++ {
		m[bytes[i][1]][bytes[i][0]] = true
	}
	visited := make(map[[2]int][3]int)
	find_path(m, Start, Start, End, visited, 0)
	res, _ := visited[End]
	result = res[0]
	end := time.Now()
	print_result(end.Sub(start), 1, fmt.Sprint(result))
}

func part_two() {
	var result [2]int
	start := time.Now()
	bytes := read_file("INPUT")
	var m [Size][Size]bool
	for i := 0; i < Rep; i++ {
		m[bytes[i][1]][bytes[i][0]] = true
	}
	t := Rep - 1
	for {
		visited := make(map[[2]int][3]int)
		find_path(m, [2]int{0, 0}, [2]int{0, 0}, [2]int{Size - 1, Size - 1}, visited, 0)
		_, ok := visited[[2]int{Size - 1, Size - 1}]
		if !ok {
			result = bytes[t]
			break
		}
		path := recreate_path(visited)
		for {
			t++
			m[bytes[t][1]][bytes[t][0]] = true
			_, on_path := path[[2]int{bytes[t][1], bytes[t][0]}]
			if on_path {
				break
			}
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, fmt.Sprintf("%d,%d", result[0], result[1]))
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "18 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
