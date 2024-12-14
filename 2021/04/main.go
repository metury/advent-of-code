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
	Blue   = "\033[1;34m"
	Yellow = "\033[1;33m"
	Green  = "\033[1;32m"
	Reset  = "\033[0m"
)

func print_result(dur time.Duration, part, result int) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

type Box struct {
	value int
	mark  bool
}

type Bingo [5][5]Box

func empty_bingo() Bingo {
	var bingo Bingo
	return bingo
}

func read_file(file_path string) ([]int, []Bingo) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	counter := 0
	var numbers []int
	var bingos []Bingo
	for _, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		space_splited := strings.Split(line, " ")
		comma_splited := strings.Split(line, ",")
		if len(comma_splited) > 1 {
			for _, nbr_string := range comma_splited {
				nr, _ := strconv.Atoi(nbr_string)
				numbers = append(numbers, nr)
			}
		} else {
			if counter%5 == 0 {
				bingos = append(bingos, empty_bingo())
			}
			j := 0
			for i := 0; i < len(space_splited); i++ {
				if len(space_splited[i]) == 0 {
					continue
				}
				nr, _ := strconv.Atoi(space_splited[i])
				bingos[counter/5][counter%5][j] = Box{nr, false}
				j++
			}
			counter++
		}
	}
	return numbers, bingos
}

func is_winning(bingo Bingo) bool {
	for i, row := range bingo {
		is_row_winning := true
		is_column_winning := true
		for j, ele := range row {
			is_row_winning = is_row_winning && ele.mark
			is_column_winning = is_column_winning && bingo[j][i].mark
		}
		if is_row_winning || is_column_winning {
			return true
		}
	}
	return false
}

func value_of_bingo(bingo Bingo) int {
	result := 0
	for _, row := range bingo {
		for _, ele := range row {
			if !ele.mark {
				result += ele.value
			}
		}
	}
	return result
}

func draw_number(bingos []Bingo, number int) (int, int, bool) {
	for k, bingo := range bingos {
		for i, row := range bingo {
			for j, ele := range row {
				if number == ele.value {
					bingos[k][i][j].mark = true
				}
			}
		}
		if is_winning(bingos[k]) {
			return number * value_of_bingo(bingos[k]), k, true
		}
	}
	return 0, -1, false
}

func part_one() {
	var result int
	start := time.Now()
	numbers, bingos := read_file("INPUT")
	for _, number := range numbers {
		val, _, win := draw_number(bingos, number)
		if win {
			result = val
			break
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	numbers, bingos := read_file("INPUT")
	for _, number := range numbers {
		val, index, win := draw_number(bingos, number)
		for win {
			bingos = append(bingos[:index], bingos[index+1:]...)
			result = val
			val, index, win = draw_number(bingos, number)
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2021" + Reset + " day " + Green + "4 - Giant Squid" + Reset)
	part_one()
	part_two()
}
