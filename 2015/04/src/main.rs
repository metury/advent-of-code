use md5::{Digest, Md5};
use std::fs;

fn read_file(filepath: &str) -> String {
    let contents = fs::read_to_string(filepath);
    let binding = contents.expect("REASON");
    binding.split('\n').next().unwrap().to_string()
}

fn prefix_md5(origin: &str, eq: &str) -> u64 {
    let mut i: u64 = 0;
    loop {
        let input = format!("{}{}", origin, i);
        let mut hasher = Md5::new();
        hasher.update(input);
        let result = hasher.finalize();
        let hash_string = format!("{:x}", result);
        if hash_string[0..eq.len()] == eq.to_string() {
            break;
        }
        i += 1;
    }
    i
}

fn part1() {
    let origin = read_file("INPUT");
    println!("Part 1: {}", prefix_md5(&origin, "00000"));
}

fn part2() {
    let origin = read_file("INPUT");
    println!("Part 2: {}", prefix_md5(&origin, "000000"));
}

fn main() {
    println!("Year 2015 day 4 - The Ideal Stocking Stuffer");
    part1();
    part2();
}
