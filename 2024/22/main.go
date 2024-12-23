package main

import (
	"fmt"
	"log"
	"os"
	"regexp"
	"strconv"
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

func read_file(file_path string) []int {
	content, err := os.ReadFile(file_path)
	if err != nil {
		log.Fatal(err)
	}
	regex := regexp.MustCompile("([0-9]+)")
	var secrets []int
	for _, find := range regex.FindAllStringSubmatch(string(content), -1) {
		nr, _ := strconv.Atoi(find[1])
		secrets = append(secrets, nr)
	}
	return secrets
}

func mix_and_prune(number, secret int) int {
	return (secret ^ number) % 16777216
}

func mod(number int) int {
	return ((number % 4) + 4) % 4
}

func generate(secret int) int {
	secret = mix_and_prune(secret*64, secret)
	secret = mix_and_prune(secret/32, secret)
	secret = mix_and_prune(secret*2048, secret)
	return secret
}

func best_sell(secret int, big_table *map[[4]int]int) {
	table := make(map[[4]int]int)
	changes := [4]int{-10, -10, -10, -10}
	last := 0
	for i := 0; i < 2000; i++ {
		secret = generate(secret)
		changes[i%4] = (secret % 10) - last
		last = secret % 10
		if changes[3] != -10 {
			key := [4]int{changes[mod(i-3)], changes[mod(i-2)], changes[mod(i-1)], changes[mod(i)]}
			_, ok := table[key]
			if !ok {
				table[key] = secret % 10
			}
		}
	}
	for key, val := range table {
		total, ok := (*big_table)[key]
		if ok {
			(*big_table)[key] = total + val
		} else {
			(*big_table)[key] = val
		}
	}
}

func part_one() {
	var result int
	start := time.Now()
	secrets := read_file("INPUT")
	for _, secret := range secrets {
		for i := 0; i < 2000; i++ {
			secret = generate(secret)
		}
		result += secret
	}
	end := time.Now()
	print_result(end.Sub(start), 1, result)
}

func part_two() {
	var result int
	start := time.Now()
	secrets := read_file("INPUT")
	table := make(map[[4]int]int)
	for _, secret := range secrets {
		best_sell(secret, &table)
	}
	for _, val := range table {
		if val > result {
			result = val
		}
	}
	end := time.Now()
	print_result(end.Sub(start), 2, result)
}

func main() {
	fmt.Println("Year " + Green + "2024" + Reset + " day " + Green + "22 - Claw Contraption" + Reset)
	part_one()
	part_two()
}
