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
	SPACE   = 0
	FILE    = 1
	INVALID = -1
)

type Partition struct {
	content int
	size    int
	index   int
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

func init_array(size, val int) []int {
	array := make([]int, size)
	for i := range array {
		array[i] = val
	}
	return array
}

func recreate_disk(format []int) []int {
	var disk []int
	id := 0
	for i := 0; i < len(format); i++ {
		if i%2 == 0 {
			partition := init_array(format[i], id)
			id++
			disk = append(disk, partition[:]...)
		} else {
			partition := init_array(format[i], INVALID)
			disk = append(disk, partition[:]...)
		}
	}
	return disk
}

func create_my_format(format []int) []Partition {
	var prop_format []Partition
	index := 0
	for i := 0; i < len(format); i++ {
		if i%2 == 0 {
			prop_format = append(prop_format, Partition{FILE, format[i], index})
			index++
		} else {
			prop_format = append(prop_format, Partition{SPACE, format[i], INVALID})
		}
	}
	return prop_format
}

func shrink(disk *[]int) {
	a, i := 0, len(*disk)-1
	for a < i {
		if (*disk)[a] == INVALID && (*disk)[i] != INVALID {
			(*disk)[a] = (*disk)[i]
			(*disk)[i] = INVALID
			a++
			i--
		} else if (*disk)[a] != INVALID {
			a++
		} else if (*disk)[i] == INVALID {
			i--
		}
	}
}

func compact(format *[]Partition) {
	for i := 1; i < len(*format); i++ {
		if (*format)[i-1].content == SPACE && (*format)[i].content == SPACE {
			(*format)[i-1].size += (*format)[i].size
			*format = append((*format)[:i], (*format)[i+1:]...)
			i--
		}
	}
}

func insert(format *[]Partition, index int, part Partition) {
	new_format := make([]Partition, len(*format)+1)
	for j := 0; j < index+1; j++ {
		new_format[j] = (*format)[j]
	}
	new_format[index+1] = part
	for j := index + 2; j < len(new_format); j++ {
		new_format[j] = (*format)[j-1]
	}
	*format = new_format
}

func reallocate_file(file_index int, format *[]Partition) {
	for a := 0; a < file_index; a++ {
		rest := (*format)[a].size - (*format)[file_index].size
		if (*format)[a].content == SPACE && rest >= 0 {
			(*format)[a].content = FILE
			(*format)[a].size = (*format)[file_index].size
			(*format)[a].index = (*format)[file_index].index
			(*format)[file_index].content = SPACE
			if rest > 0 {
				insert(format, a, Partition{SPACE, rest, -1})
				compact(format)
			}
			return
		}
	}
}

func reallocate(format *[]Partition) {
	for i := len(*format) - 1; i >= 0; i-- {
		if (*format)[i].content == FILE {
			reallocate_file(i, format)
		}
	}
}

func part_one() {
	var result int
	start := time.Now()
	disk := recreate_disk(read_file("INPUT"))
	shrink(&disk)
	for i, el := range disk {
		if el != INVALID {
			result += i * el
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	start := time.Now()
	format := create_my_format(read_file("INPUT"))
	reallocate(&format)
	position := 0
	for i := 0; i < len(format); i++ {
		for j := 0; j < format[i].size; j++ {
			if format[i].content == FILE {
				result += format[i].index * position
			}
			position++
		}
	}
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 9 - Disk Fragmenter")
	part_one()
	part_two()
}
