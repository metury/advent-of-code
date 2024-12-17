package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"regexp"
	"strconv"
	"strings"
	"time"
)

const (
	Blue   = "[1;34m"
	Yellow = "[1;33m"
	Green  = "[1;32m"
	Reset  = "[0m"
)

func print_result(dur time.Duration, part int, result string) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + result + Reset)
}

type Computer struct {
	a       int
	b       int
	c       int
	program []int
	output  []int
}

func read_file(file_path string) Computer {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	a_regex := regexp.MustCompile("Register A: ([0-9]+)")
	b_regex := regexp.MustCompile("Register B: ([0-9]+)")
	c_regex := regexp.MustCompile("Register C: ([0-9]+)")
	prg_regex := regexp.MustCompile("Program: ([0-9,]+)")
	a, _ := strconv.Atoi(a_regex.FindStringSubmatch(string(content))[1])
	b, _ := strconv.Atoi(b_regex.FindStringSubmatch(string(content))[1])
	c, _ := strconv.Atoi(c_regex.FindStringSubmatch(string(content))[1])
	var program []int
	for _, chr := range strings.Split(prg_regex.FindStringSubmatch(string(content))[1], ",") {
		nr, _ := strconv.Atoi(chr)
		program = append(program, nr)
	}
	return Computer{a, b, c, program, make([]int, 0)}
}

func combo(computer *Computer, val int) int {
	if val >= 0 && val <= 3 {
		return val
	}
	if val == 4 {
		return (*computer).a
	}
	if val == 5 {
		return (*computer).b
	}
	if val == 6 {
		return (*computer).c
	}
	panic("WHOA")
}

func do_operation(computer *Computer, pointer int) (int, bool) {
	if pointer >= len((*computer).program) {
		return -1, true
	}
	operation := (*computer).program[pointer]
	operand := (*computer).program[pointer+1]
	switch operation {
	case 0:
		(*computer).a = (*computer).a / int(math.Pow(2, float64(combo(computer, operand))))
	case 1:
		(*computer).b = (*computer).b ^ operand
	case 2:
		(*computer).b = combo(computer, operand) % 8
	case 3:
		if (*computer).a != 0 {
			return operand, false
		}
	case 4:
		(*computer).b = (*computer).b ^ (*computer).c
	case 5:
		(*computer).output = append((*computer).output, combo(computer, operand)%8)
	case 6:
		(*computer).b = (*computer).a / int(math.Pow(2, float64(combo(computer, operand))))
	case 7:
		(*computer).c = (*computer).a / int(math.Pow(2, float64(combo(computer, operand))))
	}
	return pointer + 2, false
}

func simulate(computer *Computer) string {
	pointer := 0
	halt := false
	for ; !halt; pointer, halt = do_operation(computer, pointer) {
	}
	str_output := make([]string, len((*computer).output))
	for i, out := range (*computer).output {
		str_output[i] = fmt.Sprint(out)
	}
	return strings.Join(str_output, ",")
}

func find_divisor(computer Computer) int {
	divisor := 1
	for i := 0; i < len(computer.program); i += 2 {
		if computer.program[i] == 0 {
			divisor *= int(math.Pow(2, float64(combo(&computer, computer.program[i+1]))))
		}
	}
	return divisor
}

func solve(computer Computer, prev, index, divisor int) (int, bool) {
	j := index
	for i := 0; i < divisor; i++ {
		computer.a = prev*8 + i
		computer.output = make([]int, 0)
		simulate(&computer)
		if j >= len(computer.output) {
			continue
		}
		if computer.output[len(computer.output)-j-1] == computer.program[len(computer.program)-j-1] {
			if j == len(computer.program)-1 {
				return prev*8 + i, true
			}
			sol, ok := solve(computer, prev*8+i, index+1, divisor)
			if ok {
				return sol, ok
			}
		}
	}
	return 0, false
}

func part_one() {
	var result string
	start := time.Now()
	computer := read_file("INPUT")
	result = simulate(&computer)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result string
	start := time.Now()
	computer := read_file("INPUT")
	sol, _ := solve(computer, 0, 0, find_divisor(computer))
	result = fmt.Sprint(sol)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "17 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
