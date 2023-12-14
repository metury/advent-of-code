use std::fs;

const NOT_FREE : usize = usize::MAX;

fn read_file(filepath: &str) -> Vec<Vec<char>>{
	let contents = fs::read_to_string(filepath);
	let mut matrix: Vec<Vec<char>> = vec![];
	let mut line: Vec<char> = vec!();
	for c in contents.expect("REASON").chars(){
		if c != '\n'{
			line.push(c);
		}
		else{
			matrix.push(line.clone());
			line.clear();
		}
	}
	return matrix;
}

fn compute_position(row : usize, column : usize, by_row : bool, shift : i32, i : usize) -> (usize, usize){
	if by_row{
		if shift > 0{
			return (row + i, column);
		}
		else{
			return (row - i, column);
		}
	}
	if shift > 0{
		return (row, column + i);
	}
	else{
		return (row, column - i);
	}
}

/** Change this so it takes one bool and one shift. */
fn shift_array(matrix: &mut Vec<Vec<char>>, row_start : usize, column_start : usize, row_shift: usize, column_shift: usize){
	let mut i : usize = 1;
	let mut free : usize = NOT_FREE;
	while row_start + i * row_shift < matrix.len() && column_start + i * column_shift < matrix[row_start + i * row_shift].len(){
		if matrix[row_start + i * row_shift][column_start + i * column_shift] == '.' && free == NOT_FREE{
			if row_shift == 0{
				free = column_start + i * column_shift;
			}
			else {
				free = row_start + i * row_shift;
			}
		}
		else if matrix[row_start + i * row_shift][column_start + i * column_shift] == '#'{
			free = NOT_FREE;
		}
		else if matrix[row_start + i * row_shift][column_start + i * column_shift] == 'O' && free != NOT_FREE{
			if row_shift == 0{
				matrix[row_start][free] = 'O';
				matrix[row_start][column_start + i * column_shift] = '.';
				free = free + column_shift;
			}
			else{
				matrix[free][column_start] = 'O';
				matrix[row_start + i * row_shift][column_start] = '.';
				free = free + row_shift;
			}
		}
		i += 1;
	}
}

fn main() {
	let mut matrix = read_file("../INPUT");
	println!("{:?}", matrix);
	//shift_array(&mut matrix, matrix.len() - 1, 0, -1, 0);
	//shift_array(&mut matrix, 1, 0, 0, 1);
	//shift_array(&mut matrix, 2, 0, 0, 1);
	//shift_array(&mut matrix, 3, 0, 0, 1);
	println!("{:?}", matrix);
}
