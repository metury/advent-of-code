package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
	"sync"
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

func solve(machine Machine, c chan int) {
	y := (machine.prize[1]*machine.a[0] - machine.a[1]*machine.prize[0]) / (machine.b[1]*machine.a[0] - machine.a[1]*machine.b[0])
	x := (machine.prize[0] - (machine.b[0] * y)) / machine.a[0]
	if x*machine.a[0]+y*machine.b[0] == machine.prize[0] && x*machine.a[1]+y*machine.b[1] == machine.prize[1] {
		c <- x*3 + y
	}
}

func solve_all(machines []Machine, addition int) int {
	sum := 0
	c := make(chan int, len(machines))
	var wg sync.WaitGroup
	wg.Add(len(machines))
	for _, machine := range machines {
		machine.prize[0] += addition
		machine.prize[1] += addition
		go func(machine Machine) {
			defer wg.Done()
			solve(machine, c)
		}(machine)
	}
	wg.Wait()
	close(c)
	for opt := range c{
		sum += opt
	}
	return sum
}

func part_one() {
	var result int
	start := time.Now()
	result = solve_all(read_file("INPUT"), 0)
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	result = solve_all(read_file("INPUT"), 10000000000000)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "13 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
