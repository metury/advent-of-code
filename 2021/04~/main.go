package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

type Box struct {
	value int
	mark bool
}

type Bingo [5][5]Box

func read_file(file_path string) ([]int, []Bingo) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex_numbers := regexp.MustCompile("(:?[0-9]+,)+[0-9]+")
	numbers_string := regex_numbers.FindAllString(string(content), 1)
	var numbers []int
	split_numbers := strings.Split(numbers_string[0], ",")
	for _, nbr_string := range split_numbers {
		nr, _ := strconv.Atoi(nbr_string)
		numbers = append(numbers, nr)
	}
	rows := regexp.MustCompile("([0-9]{1,2})\\s{1,2}([0-9]{1,2})\\s{1,2}([0-9]{1,2})\\s{1,2}([0-9]{1,2})\\s{1,2}([0-9]{1,2})\n")
	found_rows := rows.FindAllStringSubmatch(string(content), -1)
	var bingos []Bingo
	var bingo Bingo
	for i := 0; i < len(found_rows); i++{
		nr1, _ := strconv.Atoi(found_rows[i][1])
		nr2, _ := strconv.Atoi(found_rows[i][2])
		nr3, _ := strconv.Atoi(found_rows[i][3])
		nr4, _ := strconv.Atoi(found_rows[i][4])
		nr5, _ := strconv.Atoi(found_rows[i][5])
		bingo[i % 5] = [5]Box{{nr1, false}, {nr2, false}, {nr3, false}, {nr4, false}, {nr5, false}}
		if i % 5 == 0 && i > 0 {
			bingos = append(bingos, bingo)
		}
	}
	return numbers, bingos
}

func is_winning(bingo Bingo) bool{
	for i, row := range bingo {
		is_row_winning := true
		is_column_winning := true
		for j, ele := range row {
			is_row_winning = is_row_winning &&  ele.mark
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

func draw_number(bingos []Bingo, number int) (int, bool) {
	for k, bingo := range bingos {
		for i, row := range bingo {
			for j, ele := range  row {
				if number == ele.value {
					bingos[k][i][j].mark = true
				}
			}
		}
		if is_winning(bingo) {
			fmt.Println(bingo)
			return number * value_of_bingo(bingo), true
		}
	}
	return 0, false
}

func part1() {
	var result int
	start := time.Now()
	numbers, bingos := read_file("INPUT")
	for _, number := range numbers {
		val, win := draw_number(bingos, number)
		if win {
			result = val
			break
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part2() {
	var result int
	start := time.Now()
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2021 day 4 - Giant Squid")
	part1()
	part2()
}
