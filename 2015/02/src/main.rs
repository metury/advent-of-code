use std::cmp::min;
use std::fs;

struct Sizes {
    h: i64,
    w: i64,
    l: i64,
}

fn read_file(filepath: &str) -> Vec<Sizes> {
    let contents = fs::read_to_string(filepath);
    let mut boxes: Vec<Sizes> = vec![];
    let binding = contents.expect("REASON");
    let lines = binding.split('\n');
    for line in lines {
        if line == "" {
            continue;
        }
        let mut parts = line.split('x');
        let l = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
        let w = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
        let h = i64::from_str_radix(parts.next().unwrap(), 10).unwrap();
        boxes.push(Sizes { h: h, l: l, w: w });
    }
    return boxes;
}

fn wrapping_size(boxes: &Vec<Sizes>) -> i64 {
    let mut total: i64 = 0;
    for b in boxes {
        let area = 2 * b.l * b.w + 2 * b.l * b.h + 2 * b.h * b.w;
        let addition = min(min(b.l * b.w, b.l * b.h), b.h * b.w);
        total += area + addition;
    }
    total
}

fn ribbon_len(boxes: &Vec<Sizes>) -> i64 {
    let mut total: i64 = 0;
    for b in boxes {
        let bow = b.l * b.w * b.h;
        let mut ribbon = 0;
        if b.h <= b.w && b.h <= b.l {
            ribbon += 2 * b.h + 2 * min(b.w, b.l);
        } else if b.w <= b.h && b.w <= b.l {
            ribbon += 2 * b.w + 2 * min(b.h, b.l);
        } else {
            ribbon += 2 * b.l + 2 * min(b.w, b.h);
        }
        total += bow + ribbon;
    }
    total
}

fn part1() {
    let boxes = read_file("INPUT");
    println!("Part 1: {}", wrapping_size(&boxes));
}

fn part2() {
    let boxes = read_file("INPUT");
    println!("Part 2: {}", ribbon_len(&boxes));
}

fn main() {
    println!("Year 2015 day 02 - I Was Told There Would Be No Math");
    part1();
    part2();
}
