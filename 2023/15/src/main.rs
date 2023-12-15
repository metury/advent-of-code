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

fn insert_to_map(hash_map: &mut Vec<LinkedList<(String, i32)>>, lens: i32, key: &str){
	let hash = ascii_hash(key) as usize;
	for element in &mut hash_map[hash]{
		if element.0 == key{
			element.1 = lens;
			return;
		}
	}
	hash_map[hash].push_back((key.to_string(), lens));
}

fn remove_from_map(hash_map: &mut Vec<LinkedList<(String, i32)>>, key: &str){
	let hash = ascii_hash(key) as usize;
	let mut j: usize = 0;
	for element in &mut hash_map[hash]{
		if element.0 == key{
			let mut last = hash_map[hash].split_off(j);
			last.pop_front();
			hash_map[hash].append(&mut last);
			return;
		}
		j += 1;
	}
}

fn init_hash_map(size: i32) -> Vec<LinkedList<(String, i32)>>{
	let mut hash_map: Vec<LinkedList<(String, i32)>> = vec!();
	for _ in 0..size{
		hash_map.push(LinkedList::new());
	}
	return hash_map;
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
	let mut hash_map = init_hash_map(256);
	let strings = read_file("INPUT");
	let mut total = 0;
	for string in strings{
		if string.chars().nth(string.len() - 1) == Some('-'){
			let key = &string[0..string.len() - 1];
			remove_from_map(&mut hash_map, key);
		}
		else if string.chars().nth(string.len() - 2) == Some('='){
			let lens = string.chars().nth(string.len() - 1).unwrap() as i32 - '0' as i32;
			let key = &string[0..string.len() - 2];
			insert_to_map(&mut hash_map, lens, key);
		}
	}
	for i in 0..hash_map.len(){
		let mut j = 0;
		for element in &hash_map[i]{
			total += (i + 1) * (j + 1) * (element.1 as usize);
			j += 1;
		}
	}
	println!("Part 2: {}", total);
}

fn main() {
	part1();
	part2();
}
