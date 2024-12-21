package main

import (
	"fmt"
	"log"
	"os"
	"sort"
	"strconv"
	"strings"
	"sync"
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

const Invalid rune = 'Q'

var Numpad [][]rune = [][]rune{
	{'Q', 'Q', 'Q', 'Q', 'Q'},
	{'Q', '7', '8', '9', 'Q'},
	{'Q', '4', '5', '6', 'Q'},
	{'Q', '1', '2', '3', 'Q'},
	{'Q', 'Q', '0', 'A', 'Q'},
	{'Q', 'Q', 'Q', 'Q', 'Q'},
}

var Directionalpad = [][]rune {
	{'Q', 'Q', 'Q', 'Q', 'Q'},
	{'Q', 'Q', '^', 'A', 'Q'},
	{'Q', '<', 'v', '>', 'Q'},
	{'Q', 'Q', 'Q', 'Q', 'Q'},
}

var order = map[rune]int {
	'v': 1,
	'<': 2,
	'>': 3,
	'^': 4,
	'A': 5,
}


func customLess(a, b rune) bool {
	return order[a] < order[b]
}

func read_file(file_path string) []string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var codes []string
	for _, line := range strings.Split(string(content), "\n") {
		if len(line) == 0 {
			continue
		}
		codes = append(codes, line)
	}
	return codes
}

func append_neighbours(start, current [2]int, instr string, table map[[2]rune]string, pad [][]rune) {
	c_start := pad[start[0]][start[1]]
	c_current := pad[current[0]][current[1]]
	prev, ok := table[[2]rune{c_start, c_current}]
	if ok && len(prev) <= len(instr){
		return
	}
	table[[2]rune{c_start, c_current}] = instr + "A"
	new_current := [2]int{current[0] + 1, current[1]}
	if pad[new_current[0]][new_current[1]] != Invalid  {
		append_neighbours(start, new_current, instr + "v", table, pad)
	}
	new_current = [2]int{current[0] - 1, current[1]}
	if pad[new_current[0]][new_current[1]] != Invalid  {
		append_neighbours(start, new_current, instr + "^", table, pad)
	}
	new_current = [2]int{current[0], current[1] + 1}
	if pad[new_current[0]][new_current[1]] != Invalid  {
		append_neighbours(start, new_current, instr + ">", table, pad)
	}
	new_current = [2]int{current[0], current[1] - 1}
	if pad[new_current[0]][new_current[1]] != Invalid  {
		append_neighbours(start, new_current, instr + "<", table, pad)
	}
}

func tabelize(pad [][]rune) map[[2]rune]string {
	table := make(map[[2]rune]string)
	for i, row := range pad {
		for j := range row {
			if pad[i][j] != Invalid {
				append_neighbours([2]int{i,j}, [2]int{i,j}, "", table, pad)
			}
		}
	}
	for key, val := range table {
		runes := []rune(val)
		sort.Slice(runes, func(i, j int) bool {
			return customLess(runes[i], runes[j])
		})
		table[key] = string(runes)
	}
	return table
}

func find(table map[[2]rune]string, instr string) string {
	instr = "A" + instr
	movement := ""
	for i := 1; i < len(instr); i++{
		movement += table[[2]rune{rune(instr[i-1]), rune(instr[i])}]
	}
	runes := []rune(movement)
	if runes[0] == '<' && runes[1] == '<' {
		i := 0
		for ;runes[i] == '<'; i++ {}
		j := i + 1
		for ;runes[j] == runes[i]; j++ {}
		for k := 0; k < i && i+k < j; k++ {
			runes[k], runes[j-1-k] =  runes[j-k-1], runes[k]
		}
	}
	movement = string(runes)
	return movement
}

func repair(intr string, pad [][]rune, i, j int) string {
	res := []rune(intr)
	for k := 0; k < len(res); k++ {
		chr := res[k]
		switch chr {
			case '<':
				if pad[i][j-1] == Invalid {
					res[k+1], res[k] = res[k], res[k+1]
					k--
				} else {
					j--
				}
			case '>':
				if pad[i][j+1] == Invalid {
					res[k+1], res[k] = res[k], res[k+1]
					k--
				} else {
					j++
				}
			case '^':
				if pad[i-1][j] == Invalid {
					res[k+1], res[k] = res[k], res[k+1]
					k--
				} else {
					i--
				}
			case 'v':
				if pad[i+1][j] == Invalid {
					res[k+1], res[k] = res[k], res[k+1]
					k--
				} else {
					i++
				}
		}
	}
	return string(res)
}

func squish(instr string) string {
	runes := []rune(instr)
	for i := 1; i < len(runes) - 1; i++ {
		if runes[i] == 'A' || runes[i-1] == 'A' {
			continue
		}
		if runes[i-1] == runes[i+1] && runes[i] != runes[i-1] {
			if runes[i-1] == '>' {
				runes[i+1], runes[i] = runes[i], runes[i+1]
			} else {
				runes[i-1], runes[i] = runes[i], runes[i-1]
			}

		}
	}
	return string(runes)
}

func seq(top, n, value int, code string, table map[[2]rune]string, table_d map[[2]rune]string) int {
	fmt.Println(n)
	if n == 0 {
		return value * len(code)
	}
	if top == n {
		m := find(table, code)
		m = repair(m, Numpad, 4, 3)
		m = squish(m)
		return seq(top, n-1, value, m, table, table_d)
	} else {
		m := find(table_d, code)
		m = repair(m, Directionalpad, 1, 3)
		m = squish(m)
		return seq(top, n-1, value, m, table, table_d)
	}
}


func part_one() {
	var result int
	start := time.Now()
	codes := read_file("INPUT")
	table := tabelize(Numpad)
	table_d := tabelize(Directionalpad)
	var wg  sync.WaitGroup
	wg.Add(len(codes))
	c := make(chan int, len(codes))
	for _, code := range codes {
		go func(code string) {
			defer wg.Done()
			nr, _ :=strconv.Atoi(code[:len(code) - 1])
			c <- seq(3, 3, nr, code, table, table_d)
		}(code)
	}
	wg.Wait()
	close(c)
	for res := range c {
		result += res
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	codes := read_file("INPUT")
	table := tabelize(Numpad)
	table_d := tabelize(Directionalpad)
	var wg  sync.WaitGroup
	wg.Add(len(codes))
	c := make(chan int, len(codes))
	for _, code := range codes {
		go func(code string) {
			defer wg.Done()
			nr, _ :=strconv.Atoi(code[:len(code) - 1])
			c <- seq(26, 26, nr, code, table, table_d)
		}(code)
	}
	wg.Wait()
	close(c)
	for res := range c {
		result += res
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "21 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
