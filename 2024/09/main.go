package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"time"
)

const (
	SPACE = 0
	FILE  = 1
)

type Format struct {
	format int
	size   int
	index  int
}

func read_file(file_path string) []int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var format []int
	line := strings.Split(string(content), "\n")[0]
	for i := 0; i < len(line); i++ {
		nr, _ := strconv.Atoi(string(line[i]))
		format = append(format, nr)
	}
	return format
}

func recreate_disk(format []int) []int {
	var disk []int
	id := 0
	for i := 0; i < len(format); i++ {
		if i%2 == 0 {
			partition := make([]int, format[i])
			for i := range partition {
				partition[i] = id
			}
			id++
			disk = append(disk, partition[:]...)
		} else {
			partition := make([]int, format[i])
			for i := range partition {
				partition[i] = -1
			}
			disk = append(disk, partition[:]...)
		}
	}
	return disk
}

func create_my_format(format []int) []Format {
	var prop_format []Format
	index := 0
	for i := 0; i < len(format); i++ {
		if i%2 == 0 {
			prop_format = append(prop_format, Format{FILE, format[i], index})
			index++
		} else {
			prop_format = append(prop_format, Format{SPACE, format[i], -1})
		}
	}
	return prop_format
}

func reallocate(disk *[]int) {
	a, i := 0, len(*disk)-1
	for a < i {
		if (*disk)[a] == -1 && (*disk)[i] != -1 {
			(*disk)[a] = (*disk)[i]
			(*disk)[i] = -1
			a++
			i--
		} else if (*disk)[a] != -1 {
			a++
		} else if (*disk)[i] == -1 {
			i--
		}
	}
}

func compact(format *[]Format) {
	for i := 1; i < len(*format); i++ {
		if (*format)[i-1].format == SPACE && (*format)[i].format == SPACE {
			(*format)[i-1].size += (*format)[i].size
			*format = append((*format)[:i], (*format)[i+1:]...)
			i--
		}
	}
}

func reallocate2(format *[]Format) {
	for i := len(*format) - 1; i >= 0; i-- {
		if (*format)[i].format == FILE {
			for a := 0; a < i; a++ {
				rest := (*format)[a].size - (*format)[i].size
				found := false
				if (*format)[a].format == SPACE && rest >= 0 {
					(*format)[a].format = FILE
					(*format)[a].size = (*format)[i].size
					(*format)[a].index = (*format)[i].index
					(*format)[i].format = SPACE
					found = true
				}
				if found && rest > 0 {
					new_format := make([]Format, len(*format)+1)
					for j := 0; j < a+1; j++ {
						new_format[j] = (*format)[j]
					}
					new_format[a+1] = Format{SPACE, rest, -1}
					for j := a + 2; j < len(new_format); j++ {
						new_format[j] = (*format)[j-1]
					}
					*format = new_format
				}
				if found {
					break
				}

			}
			compact(format)
		}
	}
}

func part1() {
	var result int
	start := time.Now()
	format := read_file("INPUT")
	disk := recreate_disk(format)
	reallocate(&disk)
	for i, el := range disk {
		if el != -1 {
			result += i * el
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	format := read_file("INPUT")
	my_format := create_my_format(format)
	reallocate2(&my_format)
	position := 0
	for i := 0; i < len(my_format); i++ {
		for j := 0; j < my_format[i].size; j++ {
			if my_format[i].format == FILE {
				result += my_format[i].index * position
			}
			position++
		}
	}
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 9 - Disk Fragmenter")
	part1()
	part2()
}
