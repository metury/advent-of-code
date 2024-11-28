use regex::Regex;
use std::fs;

const ALFA_LEN: usize = 'z' as usize - 'a' as usize + 1;

struct Room {
    name: String,
    id: i64,
    checksum: String,
}

fn shift_alfa(c: char, shift: i64) -> char {
    ((((c as i64 - 'a' as i64) + shift) % ALFA_LEN as i64) + 'a' as i64) as u8 as char
}

impl Room {
    fn new(name: String, id: i64, checksum: String) -> Self {
        Room {
            name: name,
            id: id,
            checksum: checksum,
        }
    }
    fn is_valid(&self) -> bool {
        let mut counters: [u64; ALFA_LEN] = [0; ALFA_LEN];
        for c in self.name.chars() {
            if c != '-' && c != ' ' {
                let pos: usize = c as usize - 'a' as usize;
                counters[pos] += 1;
            }
        }
        let mut found: Vec<(u64, char)> = vec![];
        for i in 0..ALFA_LEN {
            if counters[i] > 0 {
                found.push((counters[i], ('a' as u8 + i as u8) as char));
            }
        }
        found.sort_by(|(c1, _), (c2, _)| c2.cmp(&c1));
        let mut cmp: String = "".to_string();
        for f in found {
            cmp.push(f.1);
        }
        cmp[..5] == self.checksum
    }
    fn decrypt(&mut self) {
        let mut builder: String = "".to_string();
        for c in self.name.chars() {
            if c == '-' {
                builder.push(' ');
            } else {
                builder.push(shift_alfa(c, self.id));
            }
        }
        self.name = builder.clone();
    }
}

fn read_file(filepath: &str) -> Vec<Room> {
    let contents = fs::read_to_string(filepath);
    let mut rooms: Vec<Room> = vec![];
    let binding = contents.expect("REASON");
    let re = Regex::new(r"([a-z\-]+)-([0-9]+)\[([a-z]+)\]").unwrap();
    for (_, [name, id, checksum]) in re.captures_iter(&binding).map(|c| c.extract()) {
        rooms.push(Room::new(
            name.to_string(),
            id.parse().unwrap(),
            checksum.to_string(),
        ));
    }
    rooms
}

fn part1() {
    let rooms = read_file("INPUT");
    let res = rooms
        .into_iter()
        .map(|r| if r.is_valid() { r.id } else { 0 })
        .sum::<i64>();
    println!("Part 1: {}", res);
}

fn part2() {
    let mut rooms = read_file("INPUT");
    let mut res: i64 = 0;
    for room in &mut rooms {
        room.decrypt();
        if room.name == "northpole object storage" {
            res = room.id;
            break;
        }
    }
    println!("Part 2: {}", res);
}

fn main() {
    println!("Year 2016 day 4 - Security Through Obscurity");
    part1();
    part2();
}
