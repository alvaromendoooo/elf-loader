use std::{cmp, io::{self, BufRead}};


pub fn elf_magic_handler(hex_value: impl AsRef<str>) -> String {
    let mut byte_split = vec![];
    let mut current: &str = hex_value.as_ref();
    let mut result: String = String::new();
    let byte_length = 2;

    while !current.is_empty() {
        let (chunk, rest) = current.split_at(cmp::min(byte_length, current.len()));
        byte_split.push(chunk);
        current = rest;
    }

    match byte_split.get(..4) {
        Some(&["7f", "45", "4c", "46"]) => {
            let class = match byte_split.get(4) {
                Some(&"01") => "32",
                Some(&"02") => "64",
                _ => "",
            };

            let endian = match byte_split.get(5) {
                Some(&"01") => "LE",
                Some(&"02") => "BE",
                _ => "",
            };

            result = format!("ELF{} {}", class, endian);
        },
        _ => result.push_str("NOT_ELF"),
    }

    result
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }

        let parts: Vec<&str> = l.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "CHECK" => println!("{}", elf_magic_handler(parts[1])),
            _ => break
        }
    }
}