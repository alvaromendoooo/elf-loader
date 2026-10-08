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
    match byte_split[..] {
        ["7f", "45", "4c", "46", ..] => {
            // Check for the class value
            let mut class: &str = "";
            match byte_split[4] {
                "01" => class = "32",
                "02" => class = "64",
                _ => {}
            };

            // Check for endian
            let mut endian: &str = "";
            match byte_split[5] {
                "01" => endian = "LE",
                "02" => endian = "BE",
                _ => {}
            }

            result.push_str(format!("ELF{} {}", class, endian).as_str());
        },
        _ =>  result.push_str("NOT_ELF")
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