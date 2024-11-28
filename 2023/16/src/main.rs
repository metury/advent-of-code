use std::collections::HashMap;
use std::fs;

type Grid<T> = Vec<Vec<T>>;

/** Read the input file and return the matrix and empty visited map. */
fn read_file(filepath: &str) -> (Grid<char>, Grid<bool>) {
    let contents = fs::read_to_string(filepath);
    let mut matrix: Grid<char> = vec![];
    let mut visited: Grid<bool> = vec![];
    let binding = contents.expect("REASON");
    let lines = binding.split('\n');
    for line in lines {
        let mut matrix_line: Vec<char> = vec![];
        let mut visited_line: Vec<bool> = vec![];
        for c in line.chars() {
            matrix_line.push(c);
            visited_line.push(false);
        }
        matrix.push(matrix_line);
        visited.push(visited_line);
    }
    return (matrix, visited);
}

/** Get the key to the hash function. */
fn get_key(position: (usize, usize), movement: (i32, i32)) -> String {
    let mut key = String::from("");
    key += &position.0.to_string();
    key.push(':');
    key += &position.1.to_string();
    key.push('|');
    key += &movement.0.to_string();
    key.push(':');
    key += &movement.1.to_string();
    return key;
}

/** Move one tile in a matrix. Also check and if it was used do not go there again. */
fn step(
    matrix: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    position: (usize, usize),
    movement: (i32, i32),
    history: &mut HashMap<String, bool>,
) {
    if position.0 >= matrix.len() || position.1 >= matrix[position.0].len() {
        return;
    }
    let key = get_key(position, movement);
    if history.contains_key(&key) {
        return;
    }
    history.insert(key, true);
    visited[position.0][position.1] = true;
    match matrix[position.0][position.1] {
        '/' => {
            let new_movement = (-movement.1, -movement.0);
            let new_position = (
                (position.0 as i32 + new_movement.0) as usize,
                (position.1 as i32 + new_movement.1) as usize,
            );
            step(matrix, visited, new_position, new_movement, history);
        }
        '\\' => {
            let new_movement = (movement.1, movement.0);
            let new_position = (
                (position.0 as i32 + new_movement.0) as usize,
                (position.1 as i32 + new_movement.1) as usize,
            );
            step(matrix, visited, new_position, new_movement, history);
        }
        '|' => {
            if movement.0 == -1 || movement.0 == 1 {
                let new_position = (
                    (position.0 as i32 + movement.0) as usize,
                    (position.1 as i32 + movement.1) as usize,
                );
                step(matrix, visited, new_position, movement, history);
            } else {
                let mut new_position = ((position.0 as i32 + 1) as usize, position.1);
                step(matrix, visited, new_position, (1, 0), history);
                new_position = ((position.0 as i32 - 1) as usize, position.1);
                step(matrix, visited, new_position, (-1, 0), history);
            }
        }
        '-' => {
            if movement.1 == -1 || movement.1 == 1 {
                let new_position = (
                    (position.0 as i32 + movement.0) as usize,
                    (position.1 as i32 + movement.1) as usize,
                );
                step(matrix, visited, new_position, movement, history);
            } else {
                let mut new_position = (position.0, (position.1 as i32 + 1) as usize);
                step(matrix, visited, new_position, (0, 1), history);
                new_position = (position.0, (position.1 as i32 - 1) as usize);
                step(matrix, visited, new_position, (0, -1), history);
            }
        }
        '.' => {
            let new_position = (
                (position.0 as i32 + movement.0) as usize,
                (position.1 as i32 + movement.1) as usize,
            );
            step(matrix, visited, new_position, movement, history);
        }
        _ => {}
    }
}

/** Count the visited places. */
fn count_visited(visited: &Vec<Vec<bool>>) -> i64 {
    let mut total: i64 = 0;
    for line in visited {
        for c in line {
            if *c {
                total += 1;
            }
        }
    }
    return total;
}

/** Try one possibility. */
fn try_one(
    matrix: &Vec<Vec<char>>,
    visited: &Vec<Vec<bool>>,
    position: (usize, usize),
    movement: (i32, i32),
) -> i64 {
    let mut v = visited.clone();
    let mut h: HashMap<String, bool> = HashMap::new();
    step(&matrix, &mut v, position, movement, &mut h);
    return count_visited(&v);
}

/** Move in the matrix. */
fn part1() {
    let (matrix, mut visited) = read_file("INPUT");
    let mut history: HashMap<String, bool> = HashMap::new();
    step(&matrix, &mut visited, (0, 0), (0, 1), &mut history);
    println!("Part 1: {}", count_visited(&visited));
}

/** Try all posibilitie. */
fn part2() {
    let (matrix, visited) = read_file("INPUT");
    let mut max: i64 = 0;
    for i in 1..matrix.len() - 1 {
        let c = try_one(&matrix, &visited, (i, 0), (0, 1));
        if c > max {
            max = c;
        }
        let c = try_one(&matrix, &visited, (matrix.len() - 1 - i, 0), (0, -1));
        if c > max {
            max = c;
        }
    }
    for i in 1..matrix[0].len() - 1 {
        let c = try_one(&matrix, &visited, (0, i), (1, 0));
        if c > max {
            max = c;
        }
        let c = try_one(&matrix, &visited, (0, matrix.len() - 1 - i), (-1, 0));
        if c > max {
            max = c;
        }
    }
    let c = try_one(&matrix, &visited, (0, 0), (1, 0));
    if c > max {
        max = c;
    }
    let c = try_one(&matrix, &visited, (0, 0), (0, 1));
    if c > max {
        max = c;
    }
    let c = try_one(&matrix, &visited, (matrix.len(), matrix[0].len()), (-1, 0));
    if c > max {
        max = c;
    }
    let c = try_one(&matrix, &visited, (matrix.len(), matrix[0].len()), (0, -1));
    if c > max {
        max = c;
    }
    println!("Part 2: {}", max);
}

fn main() {
    println!("Year 2023 day 16 - The Floor Will Be Lava");
    part1();
    part2();
}
