use std::cmp::{max, min};
use std::fs;
use std::iter::zip;

fn read_file(filepath: &str) -> Vec<Vec<i64>> {
    let contents = fs::read_to_string(filepath);
    let binding = contents.expect("REASON");
    let numbers = binding
        .split('\n')
        .filter(|c| c.len() > 0)
        .map(|c| {
            c.split('\t')
                .map(|val| val.parse::<i64>().unwrap())
                .collect()
        })
        .collect();
    numbers
}

fn part1() {
    let numbers = read_file("INPUT");
    let maximums: Vec<i64> = numbers
        .clone()
        .into_iter()
        .map(|vec| vec.into_iter().fold(0, |acc, x| max(acc, x)))
        .collect();
    let minimums: Vec<i64> = numbers
        .into_iter()
        .map(|vec| vec.into_iter().fold(i64::MAX, |acc, x| min(acc, x)))
        .collect();
    let res = zip(minimums, maximums).map(|(a, b)| b - a).sum::<i64>();
    println!("Part 1: {}", res);
}

fn part2() {
    let numbers = read_file("INPUT");
    let mut total: i64 = 0;
    for num_line in numbers {
        'line: for i in 0..num_line.len() {
            for j in i + 1..num_line.len() {
                if num_line[i] % num_line[j] == 0 {
                    total += num_line[i] / num_line[j];
                    break 'line;
                } else if num_line[j] % num_line[i] == 0 {
                    total += num_line[j] / num_line[i];
                    break 'line;
                }
            }
        }
    }
    println!("Part 2: {}", total);
}

fn main() {
    println!("Year 2017 day 2 - Corruption Checksum");
    part1();
    part2();
}
