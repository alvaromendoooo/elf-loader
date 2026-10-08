use std::{cmp, io::{self, BufRead}};


pub fn elf_e_type(byte_split: &Vec<&str>) -> &'static str {
    let mut hex_str = String::new();
    
    if let Some(&type_first) = byte_split.get(16) {
        if let Some(&type_second) = byte_split.get(17) {
            hex_str = format!("{}{}", type_first, type_second);
        }
    }

    if hex_str.len() != 4 {
        return "OTHER";
    }

    let b0 = u8::from_str_radix(&hex_str[0..2], 16).ok();
    let b1 = u8::from_str_radix(&hex_str[2..4], 16).ok();

    match (b0, b1) {
        (Some(byte16), Some(byte17)) => {
            let val = u16::from_le_bytes([byte16, byte17]);

            match val {
                1 => "REL",
                2 => "EXEC",
                3 => "DYN",
                4 => "CORE",
                _ => "OTHER",
            }
        }
        _ => "OTHER",
    }
}

pub fn elf_header_handler(hex_value: impl AsRef<str>) -> String {
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
            let elf_class = match byte_split.get(4) {
                Some(&"01") => "32",
                Some(&"02") => "64",
                _ => "UNKNOWN",
            };

            let elf_endian = match byte_split.get(5) {
                Some(&"01") => "LE",
                Some(&"02") => "BE",
                _ => "UNKNOWN",
            };

            let elf_type = elf_e_type(&byte_split);

            result = format!("CLASS {}\nENDIAN {}\nTYPE {}", elf_class, elf_endian, elf_type);
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

        /*match parts[0] {
            "CHECK" => println!("{}", elf_magic_handler(parts[1])),
            _ => break
        }*/
        println!("{}", elf_header_handler(parts[0]));
    }
}