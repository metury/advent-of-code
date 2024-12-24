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

func read_file(file_path string) ([]Gate, map[string]bool, map[string][2]string) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	inits := regexp.MustCompile("([0-9a-z]+): (0|1)")
	and_gates := regexp.MustCompile("([0-9a-z]+) AND ([0-9a-z]+) \\-> ([0-9a-z]+)")
	or_gates := regexp.MustCompile("([0-9a-z]+) OR ([0-9a-z]+) \\-> ([0-9a-z]+)")
	xor_gates := regexp.MustCompile("([0-9a-z]+) XOR ([0-9a-z]+) \\-> ([0-9a-z]+)")
	values := make(map[string]bool)
	var gates []Gate
	reverse_gates := make(map[string][2]string)
	for _, found := range inits.FindAllStringSubmatch(string(content), -1) {
		if found[2] == "1" {
			values[found[1]] = true
		} else {
			values[found[1]] = false
		}

	}
	for _, found := range and_gates.FindAllStringSubmatch(string(content), -1) {
		gates = append(gates,Gate {[2]string{found[1], found[2]}, found[3], And, false})
		reverse_gates[found[3]] = [2]string{found[1], found[2]}
	}
	for _, found := range or_gates.FindAllStringSubmatch(string(content), -1) {
		gates = append(gates,Gate {[2]string{found[1], found[2]}, found[3], Or, false})
		reverse_gates[found[3]] = [2]string{found[1], found[2]}
	}
	for _, found := range xor_gates.FindAllStringSubmatch(string(content), -1) {
		gates = append(gates,Gate {[2]string{found[1], found[2]}, found[3], Xor, false})
		reverse_gates[found[3]] = [2]string{found[1], found[2]}
	}
	return gates, values, reverse_gates
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
	done := true
	for done {
		done = false
		for i, gate := range *gates {
			if gate.evaluated {
				continue
			}
			done = true
			first, ok := (*values)[gate.inputs[0]]
			second, sok := (*values)[gate.inputs[1]]
			if ok && sok {
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

func print_stack(gate string, reverse_gates *map[string][2]string, bad *map[string]bool) {
	current := []string{gate}
	for len(current) > 0{
		val, ok := (*reverse_gates)[current[0]]
		if ok {
			(*bad)[current[0]] = true
			current = append(current[1:], val[:]...)

		} else {
			current = current[1:]
		}
	}
}


func find_four(bad *[]string, all *[][4]string, current [4]string, index, global int) {
	if global == len(*bad) {
		return
	}
	if index == 4 {
		*all = append(*all, current)
		return
	}
	find_four(bad, all, current, index, global+1)
	current[index] = (*bad)[global]
	find_four(bad, all, current, index+1, global+1)
}

func find_mistakes(gates *[]Gate, values *map[string]bool,  reverse_gates *map[string][2]string) {
	x, bitx := eval_bit(values, 'x')
	y, bity := eval_bit(values, 'y')
	z, bitz := eval_bit(values, 'z')
	fmt.Println("Z:", z, bitz, "X:", x, bitx, "Y:", y, bity)

	bad := make(map[string]bool)

	acc := 0
	for i := 0; i < len(bitz) && i < len(bitx) && i < len(bity); i++ {
		result := bitx[i] + bity[i] + acc
		acc = result / 2
		if bitz[i] != result%2 {
			print_stack(fmt.Sprintf("z%.2d", i), reverse_gates, &bad)
		}
	}
	var bad_array []string
	for key := range bad {
		bad_array = append(bad_array, key)
	}
	var all [][4]string
	find_four(&bad_array, &all, [4]string{"", "", "", ""}, 0, 0)
	fmt.Println(all)
}

func part_one() {
	var result int
	start := time.Now()
	gates, values, _ := read_file("INPUT")
	evaluate_gates(&gates, &values)
	result, _ = eval_bit(&values, 'z')
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	gates, values, reverese_gates := read_file("INPUT")
	evaluate_gates(&gates, &values)
	find_mistakes(&gates, &values, &reverese_gates)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "24 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
