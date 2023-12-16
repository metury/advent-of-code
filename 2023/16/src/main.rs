use std::fs;
use std::collections::HashMap;

/** Read the input file and return the matrix and empty visited map. */
fn read_file(filepath: &str) -> (Vec<Vec<char>>, Vec<Vec<bool>>){
	let contents = fs::read_to_string(filepath);
	let mut matrix: Vec<Vec<char>> = vec![];
	let mut found: Vec<Vec<bool>> = vec![];
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		let mut matrix_line: Vec<char> = vec!();
		let mut found_line: Vec<bool> = vec!();
		for c in line.chars(){
			matrix_line.push(c);
			found_line.push(false);
		}
		matrix.push(matrix_line);
		found.push(found_line);
	}
	return (matrix, found);
}

/** Get the key to the hash function. */
fn get_key(position: (usize, usize), movement: (i32, i32)) -> String{
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
fn move_matrix(matrix: &Vec<Vec<char>>, found: &mut Vec<Vec<bool>>, position: (usize, usize), movement: (i32, i32), history: &mut HashMap<String, bool>){
	if position.0 >= matrix.len() || position.1 >= matrix[position.0].len(){
		return;
	}
	let key = get_key(position, movement);
	if history.contains_key(&key){
		return;
	}
	history.insert(key, true);
	found[position.0][position.1] = true;
	match matrix[position.0][position.1]{
		'/' => {
			let new_movement = (-movement.1, -movement.0);
			let new_position = ((position.0 as i32 + new_movement.0) as usize, (position.1 as i32 + new_movement.1) as usize);
			move_matrix(matrix, found, new_position, new_movement, history);
			},
		'\\' => {
			let new_movement = (movement.1, movement.0);
			let new_position = ((position.0 as i32 + new_movement.0) as usize, (position.1 as i32 + new_movement.1) as usize);
			move_matrix(matrix, found, new_position, new_movement, history);
			},
		'|' => {
			if movement.0 == -1 || movement.0 == 1{
					let new_position = ((position.0 as i32 + movement.0) as usize, (position.1 as i32 + movement.1) as usize);
					move_matrix(matrix, found, new_position, movement, history);
				}
				else{
					let mut new_position = ((position.0 as i32 + 1) as usize, position.1);
					move_matrix(matrix, found, new_position, (1, 0), history);
					new_position = ((position.0 as i32 - 1) as usize, position.1);
					move_matrix(matrix, found, new_position, (-1, 0), history);
				}
			},
		'-' => {
				if movement.1 == -1 || movement.1 == 1{
					let new_position = ((position.0 as i32 + movement.0) as usize, (position.1 as i32 + movement.1) as usize);
					move_matrix(matrix, found, new_position, movement, history);
				}
				else{
					let mut new_position = (position.0, (position.1 as i32 + 1) as usize);
					move_matrix(matrix, found, new_position, (0, 1), history);
					new_position = (position.0, (position.1 as i32 - 1) as usize);
					move_matrix(matrix, found, new_position, (0, -1), history);
				}
			},
		'.' => {
			let new_position = ((position.0 as i32 + movement.0) as usize, (position.1 as i32 + movement.1) as usize);
			move_matrix(matrix, found, new_position, movement, history);
			},
		_ => {},
	}
}

/** Count the visited places. */
fn count_visited(found: &Vec<Vec<bool>>) -> i64{
	let mut total: i64 = 0;
	for line in found{
		for c in line{
			if *c{
				total += 1;
			}
		}
	}
	return total;
}

/** Try one possibility. */
fn try_one(matrix: & Vec<Vec<char>>, found: &Vec<Vec<bool>>, position: (usize, usize), movement: (i32, i32)) -> i64{
	let mut f = found.clone();
	let mut h: HashMap<String, bool> = HashMap::new();
	move_matrix(&matrix, &mut f, position, movement, &mut h);
	return count_visited(&f);
}

/** Move in the matrix. */
fn part1(){
	let (matrix, mut found) = read_file("INPUT");
	let mut history: HashMap<String, bool> = HashMap::new();
	move_matrix(&matrix, &mut found, (0,0), (0,1), &mut history);
	println!("Part 1: {}", count_visited(&found));
}

/** Try all posibilitie. */
fn part2(){
	let (matrix, found) = read_file("INPUT");
	let mut max: i64 = 0;
	for i in 1..matrix.len() - 1{
		let c = try_one(&matrix, &found, (i,0), (0,1));
		if c > max{ max = c; }
		let c = try_one(&matrix, &found, (matrix.len() - 1 - i,0), (0,-1));
		if c > max{ max = c; }
	}
	for i in 1..matrix[0].len() - 1{
		let c = try_one(&matrix, &found, (0,i), (1,0));
		if c > max{ max = c; }
		let c = try_one(&matrix, &found, (0,matrix.len() - 1 - i), (-1,0));
		if c > max{ max = c; }
	}
	let c = try_one(&matrix, &found, (0,0), (1,0));
	if c > max{ max = c; }
	let c = try_one(&matrix, &found, (0,0), (0,1));
	if c > max{ max = c; }
	let c = try_one(&matrix, &found, (matrix.len(),matrix[0].len()), (-1,0));
	if c > max{ max = c; }
	let c = try_one(&matrix, &found, (matrix.len(),matrix[0].len()), (0,-1));
	if c > max{ max = c; }
	println!("Part 2: {}", max);
}

fn main() {
	part1();
	part2();
}
