use std::fs;

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

struct Node{
	lens: i32,
	next: Box<Node>,
}

struct AsciiHasMap{
	boxes: [Node; 256],
}

impl Node{
	fn add_node(lens: i32){
		let mut node = Self.next;
		while node.lens != -1{
			node = node.next;
		}
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
	
}

fn main() {
	part1();
	//part2();
}
