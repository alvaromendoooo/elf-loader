use std::io::{self, BufRead};

// TODO (what-is-elf): implement per the lesson description.

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        println!("TODO");
    }
}
