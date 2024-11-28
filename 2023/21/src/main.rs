use std::collections::{HashSet, VecDeque};
use std::fs;

type Grid<T> = Vec<Vec<T>>;
type Position = (usize, usize);

const ROCK: i64 = -3;
const EMPTY: i64 = -1;

fn read_file(filepath: &str) -> (Grid<bool>, Position) {
    let contents = fs::read_to_string(filepath);
    let mut garden: Grid<bool> = vec![];
    let mut start: Position = (0, 0);
    let binding = contents.expect("REASON");
    let lines = binding.split('\n');
    let mut i: usize = 0;
    for line in lines {
        let mut garden_line: Vec<bool> = vec![];
        let mut j: usize = 0;
        for c in line.chars() {
            garden_line.push(c != '#');
            if c == 'S' {
                start = (i, j);
            }
            j += 1;
        }
        if garden_line.len() > 0 {
            garden.push(garden_line);
        }
        i += 1;
    }
    (garden, start)
}

fn add(a: usize, b: i8) -> usize {
    (a as i64 + b as i64) as usize
}

fn bfs(garden: &Grid<bool>, position: Position) -> Grid<i64> {
    let mut distances: Grid<i64> = vec![];
    for g in garden {
        distances.push(vec![EMPTY; g.len()])
    }

    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(position);
    let neighbors: [(i8, i8); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut queue: VecDeque<(Position, i64)> = VecDeque::new();
    queue.push_back((position, 0));

    while !queue.is_empty() {
        let (pos, counter) = queue.pop_front().unwrap();
        if !garden[pos.0][pos.1] {
            distances[pos.0][pos.1] = ROCK;
            continue;
        } else {
            distances[pos.0][pos.1] = counter;
        }
        for n in neighbors {
            let new_pos = (add(pos.0, n.0), add(pos.1, n.1));
            if new_pos.0 >= garden.len() || new_pos.1 >= garden[new_pos.0].len() {
                continue;
            }
            if !visited.contains(&new_pos) {
                queue.push_back((new_pos, counter + 1));
                visited.insert(new_pos);
            }
        }
    }
    distances
}

fn part1() {
    let (garden, start) = read_file("INPUT");
    let dist = bfs(&garden, start);
    let limit: i64 = 64;
    let total = dist
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .filter(|x| x % 2 == 0 && x != &ROCK && x <= &limit)
                .count()
        })
        .fold(0, |acc, sum| acc + sum);
    println!("Part 1: {}", total);
}

fn part2() {
    let (garden, start) = read_file("INPUT");
    let dist = bfs(&garden, start);
    let magical_const: usize = 26501365;
    let n: usize = magical_const / garden.len();
    let limit: i64 = magical_const as i64 % garden.len() as i64;
    let full_even = dist
        .clone()
        .into_iter()
        .map(|vec| vec.into_iter().filter(|x| x % 2 == 0 && x != &ROCK).count())
        .fold(0, |acc, sum| acc + sum);
    let full_odd = dist
        .clone()
        .into_iter()
        .map(|vec| vec.into_iter().filter(|x| x % 2 == 1 && x != &ROCK).count())
        .fold(0, |acc, sum| acc + sum);
    let corner_odd = dist
        .clone()
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .filter(|x| x % 2 == 1 && x != &ROCK && x > &limit)
                .count()
        })
        .fold(0, |acc, sum| acc + sum);
    let corner_even = dist
        .clone()
        .into_iter()
        .map(|vec| {
            vec.into_iter()
                .filter(|x| x % 2 == 0 && x != &ROCK && x > &limit)
                .count()
        })
        .fold(0, |acc, sum| acc + sum);

    let total =
        (n + 1) * (n + 1) * full_odd + n * n * full_even - (n + 1) * corner_odd + n * corner_even;
    println!("Part 2: {}", total);
}

fn main() {
    println!("Year 2023 day 21 - Step Counter");
    part1();
    part2();
}
