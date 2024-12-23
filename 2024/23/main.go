package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"sort"
	"strings"
	"time"
)

const (
	Blue   = "[1;34m"
	Yellow = "[1;33m"
	Green  = "[1;32m"
	Red    = "\033[1;31m"
	Reset  = "[0m"
)

func print_result(dur time.Duration, part int, result string) {
	fmt.Println("Part " + fmt.Sprint(part) + " [" + Blue + fmt.Sprint(dur) + Reset + "]: " + Yellow + result + Reset)
}

func read_file(file_path string) map[string][]string {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	connections := make(map[string][]string)
	regex := regexp.MustCompile("([a-z]+)\\-([a-z]+)")
	for _, found := range regex.FindAllStringSubmatch(string(content), -1) {
		connections[found[1]] = append(connections[found[1]], found[2])
		connections[found[2]] = append(connections[found[2]], found[1])
	}
	return connections
}

func find_triples(connections map[string][]string) int {
	sum := 0
	for key, val := range connections {
		for _, neighbour := range val {
			for _, third := range connections[neighbour] {
				for _, circle := range connections[third] {
					if circle == key && (third[0] == 't' || key[0] == 't' || neighbour[0] == 't') {
						sum += 1
					}
				}
			}
		}
	}
	return sum / (3 * 2)
}

func find_clique(connections map[string][]string) string {
	max := 0
	max_clique := make(map[string]bool)
	for key, val := range connections {
		for _, neighbour := range val {
			clique := make(map[string]bool, 0)
			clique[key] = true
			clique[neighbour] = true
		outer:
			for _, candidate := range connections[neighbour] {
			inner:
				for inside := range clique {
					for _, s := range connections[candidate] {
						if s == inside {
							continue inner
						}
					}
					continue outer
				}
				clique[candidate] = true
			}
			if max < len(clique) {
				max_clique = clique
				max = len(max_clique)
			}
		}
	}
	var result []string
	for key := range max_clique {
		result = append(result, key)
	}
	sort.Strings(result)
	return strings.Join(result, ",")
}

func part_one() {
	var result int
	start := time.Now()
	connections := read_file("INPUT")
	result = find_triples(connections)
	end := time.Now()
	print_result(end.Sub(start), 1, fmt.Sprint(result))
}

func part_two() {
	var result string
	start := time.Now()
	connections := read_file("INPUT")
	result = find_clique(connections)
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "23 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
