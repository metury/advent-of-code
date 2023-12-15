use std::fs;
use std::collections::LinkedList;

fn read_file(filepath: &str) -> Vec<String>{
	let contents = fs::read_to_string(filepath);
	let mut strings: Vec<String> = vec!();
	let binding = contents.expect("REASON");
	let lines = binding.split('\n');
	for line in lines{
		let inputs = line.split(',');
		for input in inputs{
			if input != ""{
				strings.push(input.to_string());
			}
		}
	}
	return strings;
}

fn ascii_hash(string: &str) -> i32{
	let mut hash: i32 = 0;
	for c in string.chars(){
		let ascii = c as i32;
		hash = ((hash + ascii) * 17) % 256;
	}
	return hash;
}

fn insert_to_map(hash_map: &mut [LinkedList<i32>; 256], lens: i32, key: &str){
	let hash = ascii_hash(key);
	let list = &hash_map[hash as usize];
	if list.contains(&hash){
		// This is actually nonsense.
	}
}


fn part1(){
	let strings = read_file("INPUT");
	let mut total = 0;
	for string in strings{
		total += ascii_hash(&string);
	}
	println!("Part 1: {}", total);
}

fn part2(){
	let mut hash_map: [LinkedList<i32>; 256] = [LinkedList::new(); 256];
	let strings = read_file("INPUT");
	let mut total = 0;
	for string in strings{
		if string.chars().nth(string.len() - 1) == Some('-'){
			let key = &string[0..string.len() - 2];
			// Remove.
		}
		else if string.chars().nth(string.len() - 2) == Some('='){
			let lens = string.chars().nth(string.len() - 1).unwrap();
			let key = &string[0..string.len() - 3];
			// Insert
		}
	}
	for i in 0..hash_map.len(){
		let mut j = 0;
		for element in hash_map[i]{
			total += (i + 1) * (j + 1) * (element as usize);
			j += 1;
		}
	}
	println!("Part 2: {}", total);
}

fn main() {
	part1();
	part2();
}
