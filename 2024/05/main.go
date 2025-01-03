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

const (
	Blue   = "\033[1;34m"
	Yellow = "\033[1;33m"
	Green  = "\033[1;32m"
	Reset  = "\033[0m"
)

func print_result(dur time.Duration, part, result int) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + fmt.Sprint(result) + Reset)
}

func read_file(file_path string) (map[int][]int, [][]int) {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	rules_regex := regexp.MustCompile("([0-9]+)\\|([0-9]+)")
	ordering_regex := regexp.MustCompile("(:?[0-9]+,)+[0-9]+")
	rules := make(map[int][]int)
	found_rules := rules_regex.FindAllStringSubmatch(string(content), -1)
	for _, rule := range found_rules {
		first, _ := strconv.Atoi(rule[1])
		second, _ := strconv.Atoi(rule[2])
		val, ok := rules[first]
		if ok {
			rules[first] = append(val, second)
		} else {
			rules[first] = []int{second}
		}
	}
	var orders [][]int
	found_orders := ordering_regex.FindAllString(string(content), -1)
	for _, order := range found_orders {
		splited := strings.Split(order, ",")
		var order_nr []int
		for _, s := range splited {
			number, _ := strconv.Atoi(s)
			order_nr = append(order_nr, number)
		}
		orders = append(orders, order_nr)
	}
	return rules, orders
}

func filter(rules map[int][]int, orders [][]int) ([][]int, [][]int) {
	var correct [][]int
	var incorrect [][]int
outer:
	for _, order := range orders {
		for i, page := range order {
			rule, ok := rules[page]
			if ok {
				for _, prev := range order[:i] {
					for _, r := range rule {
						if r == prev {
							incorrect = append(incorrect, order)
							continue outer
						}
					}
				}
			}
		}
		correct = append(correct, order)
	}
	return correct, incorrect
}

func fix(rules map[int][]int, order []int) []int {
	for k := 0; k < len(order); k++ {
		var wrong []int
		rule, ok := rules[order[k]]
		if ok {
			for j := 0; j < k; j++ {
				prev := order[j]
				for _, r := range rule {
					if prev == r {
						wrong = append(wrong, prev)
						order = append(order[:j], order[j+1:]...)
						k--
						j--
					}
				}
			}
		}
		if len(wrong) > 0 {
			var fixed []int
			fixed = append(fixed, order[:k+1]...)
			fixed = append(fixed, wrong[:]...)
			fixed = append(fixed, order[k+1:]...)
			order = fixed
		}
	}
	return order
}

func part_one() {
	var result int
	rules, orders := read_file("INPUT")
	start := time.Now()
	correct, _ := filter(rules, orders)
	for _, cor := range correct {
		result += cor[len(cor)/2]
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	rules, orders := read_file("INPUT")
	start := time.Now()
	_, incorrect := filter(rules, orders)
	for _, inc := range incorrect {
		result += fix(rules, inc)[len(inc)/2]
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "5 - Print Queue" + Reset)
	part_one()
	part_two()
}
