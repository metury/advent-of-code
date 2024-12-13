package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"time"
)

const Invalid = -1

type Machine struct {
	a     [2]int
	b     [2]int
	prize [2]int
}

func read_file(file_path string) []Machine {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	var machines []Machine
	regex := regexp.MustCompile("Button A: X\\+([0-9]+), Y\\+([0-9]+)\\nButton B: X\\+([0-9]+), Y\\+([0-9]+)\\nPrize: X=([0-9]+), Y=([0-9]+)")
	found := regex.FindAllStringSubmatch(string(content), -1)
	for _, f := range found {
		a_x, _ := strconv.Atoi(f[1])
		a_y, _ := strconv.Atoi(f[2])
		b_x, _ := strconv.Atoi(f[3])
		b_y, _ := strconv.Atoi(f[4])
		prize_x, _ := strconv.Atoi(f[5])
		prize_y, _ := strconv.Atoi(f[6])
		machine := Machine{
			a:     [2]int{a_x, a_y},
			b:     [2]int{b_x, b_y},
			prize: [2]int{prize_x, prize_y},
		}
		machines = append(machines, machine)
	}
	return machines
}

func solve(machine Machine) int {
	y := (machine.prize[1]*machine.a[0] - machine.a[1]*machine.prize[0]) / (machine.b[1]*machine.a[0] - machine.a[1]*machine.b[0])
	x := (machine.prize[0] - (machine.b[0] * y)) / machine.a[0]
	if x*machine.a[0]+y*machine.b[0] == machine.prize[0] && x*machine.a[1]+y*machine.b[1] == machine.prize[1] {
		return x*3 + y
	}
	return Invalid
}

func part_one() {
	var result int
	start := time.Now()
	machines := read_file("INPUT")
	for _, machine := range machines {
		opt := solve(machine)
		if opt != Invalid {
			result += opt
		}
	}
	end := time.Now()
	fmt.Println("Part 1 [", end.Sub(start), "]:", result)
}

func part_two() {
	var result int
	start := time.Now()
	machines := read_file("INPUT")
	for _, machine := range machines {
		machine.prize[0] += 10000000000000
		machine.prize[1] += 10000000000000
		opt := solve(machine)
		if opt != Invalid {
			result += opt
		}
	}
	end := time.Now()
	fmt.Println("Part 2 [", end.Sub(start), "]:", result)
}

func main() {
	fmt.Println("Year 2024 day 13 - Claw Contraption")
	part_one()
	part_two()
}
