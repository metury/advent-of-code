use std::collections::HashMap;
use std::fs;

const NOT_FREE: usize = usize::MAX;

#[derive(PartialEq)]
enum Side {
    North,
    South,
    West,
    East,
}

fn read_file(filepath: &str) -> Vec<Vec<char>> {
    let contents = fs::read_to_string(filepath);
    let mut matrix: Vec<Vec<char>> = vec![];
    let mut line: Vec<char> = vec![];
    for c in contents.expect("REASON").chars() {
        if c != '\n' {
            line.push(c);
        } else {
            matrix.push(line.clone());
            line.clear();
        }
    }
    return matrix;
}

fn compute_position(
    row: usize,
    column: usize,
    by_row: bool,
    reverse: bool,
    shift: usize,
) -> (usize, usize, bool) {
    if by_row {
        if !reverse {
            return (row + shift, column, false);
        } else {
            return (row - shift, column, row - shift == 0);
        }
    }
    if !reverse {
        return (row, column + shift, false);
    } else {
        return (row, column - shift, column - shift == 0);
    }
}

fn shift_array(
    matrix: &mut Vec<Vec<char>>,
    row_start: usize,
    column_start: usize,
    by_row: bool,
    reverse: bool,
) {
    let mut shift: usize = 0;
    let mut free: usize = NOT_FREE;
    let (mut row, mut column, mut last) =
        compute_position(row_start, column_start, by_row, reverse, shift);
    while row < matrix.len() && column < matrix[row].len() {
        if matrix[row][column] == '.' && free == NOT_FREE {
            if by_row {
                free = row;
            } else {
                free = column;
            }
        } else if matrix[row][column] == '#' {
            free = NOT_FREE;
        } else if matrix[row][column] == 'O' && free != NOT_FREE {
            if by_row {
                matrix[free][column] = 'O';
                matrix[row][column] = '.';
            } else {
                matrix[row][free] = 'O';
                matrix[row][column] = '.';
            }
            if reverse {
                free = free - 1;
            } else {
                free = free + 1;
            }
        }
        shift += 1;
        if last {
            return;
        }
        (row, column, last) = compute_position(row_start, column_start, by_row, reverse, shift);
    }
}

fn shift_side(matrix: &mut Vec<Vec<char>>, side: Side) {
    match side {
        Side::North => {
            for i in 0..matrix[0].len() {
                shift_array(matrix, 0, i, true, false);
            }
        }
        Side::South => {
            for i in 0..matrix[0].len() {
                shift_array(matrix, matrix.len() - 1, i, true, true);
            }
        }
        Side::East => {
            for i in 0..matrix[0].len() {
                shift_array(matrix, i, matrix[i].len() - 1, false, true);
            }
        }
        Side::West => {
            for i in 0..matrix[0].len() {
                shift_array(matrix, i, 0, false, false);
            }
        }
    }
}

fn cycle(matrix: &mut Vec<Vec<char>>) {
    shift_side(matrix, Side::North);
    shift_side(matrix, Side::West);
    shift_side(matrix, Side::South);
    shift_side(matrix, Side::East);
}

fn compute(matrix: &Vec<Vec<char>>) -> usize {
    let size: usize = matrix.len();
    let mut ret: usize = 0;
    for i in 0..size {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 'O' {
                ret += size;
                ret -= i
            }
        }
    }
    return ret;
}

fn insert_to_history(
    matrix: &Vec<Vec<char>>,
    history: &mut HashMap<String, i64>,
    cycle: i64,
) -> i64 {
    let mut key: String = String::from("");
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            key.push(matrix[i][j]);
        }
    }
    if history.contains_key(&key) {
        return history[&key];
    }
    history.insert(key, cycle);
    return -1;
}

fn part1() {
    let mut matrix = read_file("INPUT");
    shift_side(&mut matrix, Side::North);
    println!("Part 1: {}", compute(&matrix));
}

fn part2() {
    let mut matrix = read_file("INPUT");
    let mut history: HashMap<String, i64> = HashMap::new();
    let cycles: i64 = 1_000_000_000;
    for i in 0..cycles {
        cycle(&mut matrix);
        let rep: i64 = insert_to_history(&matrix, &mut history, i);
        if rep != -1 {
            let m: i64 = (1_000_000_000 - rep) % (i - rep) - 1;
            for _ in 0..m {
                cycle(&mut matrix);
            }
            println!("Part 2: {}", compute(&matrix));
            return;
        }
    }
    println!("Part 2: {}", compute(&matrix));
}

fn main() {
    println!("Year 2023 day 14 - Parabolic Reflector Dish");
    part1();
    part2();
}
