package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
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

const (
	And = 0
	Or = 1
	Xor = 2
)

type Gate struct {
	inputs [2]string
	output string
	gate int
	evaluated bool
}

func read_file(file_path string) ([]Gate, map[string]bool) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	inits := regexp.MustCompile("([0-9a-z]+): (0|1)")
	regex_gates := regexp.MustCompile("([0-9a-z]+) ((?:AND)|(?:OR)|(?:XOR)) ([0-9a-z]+) \\-> ([0-9a-z]+)")
	values := make(map[string]bool)
	var gates []Gate
	for _, found := range inits.FindAllStringSubmatch(string(content), -1) {
		if found[2] == "1" {
			values[found[1]] = true
		} else {
			values[found[1]] = false
		}
	}
	for _, found := range regex_gates.FindAllStringSubmatch(string(content), -1) {
		switch found[2] {
			case "OR":
				gates = append(gates,Gate {[2]string{found[1], found[3]}, found[4], Or, false})
			case "XOR":
				gates = append(gates,Gate {[2]string{found[1], found[3]}, found[4], Xor, false})
			case "AND":
				gates = append(gates,Gate {[2]string{found[1], found[3]}, found[4], And, false})
		}
	}
	return gates, values
}

func eval_bit(values *map[string]bool, first_symbol byte) (int, []int) {
	var number []string
	for key := range *values {
		if key[0] == first_symbol {
			number = append(number, key)
		}
	}
	sort.Strings(number)
	var bits []int
	sum := 0
	acc := 1
	for i := 0; i < len(number); i++ {
		val, _ := (*values)[number[i]]
		if val {
			bits = append(bits, 1)
			sum += acc
		} else {
			bits = append(bits, 0)
		}
		acc = 2*acc
	}
	return sum, bits
}

func evaluate_gates(gates *[]Gate, values *map[string]bool) {
	for i := range *gates {
		(*gates)[i].evaluated = false
	}
	done := true
	for done {
		done = false
		for i, gate := range *gates {
			if gate.evaluated {
				continue
			}
			done = true
			first, first_ok := (*values)[gate.inputs[0]]
			second, second_ok := (*values)[gate.inputs[1]]
			if first_ok && second_ok {
				(*gates)[i].evaluated = true
				switch gate.gate {
					case And:
						(*values)[(*gates)[i].output] = first && second
					case Or:
						(*values)[(*gates)[i].output] = first || second
					case Xor:
						(*values)[(*gates)[i].output] = first != second
				}
			}
		}
	}
}

func find_candidates(gates []Gate) []string {
	var candidates []string
	for _, gate := range gates {
		if gate.output[0] == 'z' && gate.gate != Xor{
			candidates = append(candidates, gate.output)
		}
	}
	sort.Strings(candidates)
	return candidates[:len(candidates)-1]
}

func find_xors(gates []Gate) []string {
	var candidates []string
	for _, gate := range gates {
		if gate.output[0] != 'z' && gate.gate == Xor{
			candidates = append(candidates, gate.output)
		}
	}
	return candidates
}

func create_tuples(candidates []string, xors []string) [][2]string {
	var tuples [][2]string
	for _, candidate := range candidates {
		for _, xor := range xors {
			tuples = append(tuples, [2]string{candidate, xor})
		}
	}
	return tuples
}

func part_one() {
	var result int
	start := time.Now()
	gates, values := read_file("INPUT")
	evaluate_gates(&gates, &values)
	result, _ = eval_bit(&values, 'z')
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	gates, _ := read_file("INPUT")
	candidates := create_tuples(find_candidates(gates), find_xors(gates))
	fmt.Println(candidates, len(find_xors(gates)))
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "24 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
