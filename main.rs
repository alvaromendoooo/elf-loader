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

pub fn segment_load(virtual_address: i32, file_size: i32, memory_size: i32, flags: impl AsRef<str>) -> String {
    let mut result = String::new();
    
    result.push_str(format!("MAP {} SIZE {} PROT {}", virtual_address, memory_size, flags.as_ref()).as_str());

    if memory_size > file_size {
        let bss_start = virtual_address + file_size;
        let bss_size = memory_size - file_size;
        result.push_str(format!("\nBSS {} {}", bss_start, bss_size).as_str());
    }
    
    result
}

pub fn lay_out_args(args: impl AsRef<str>) -> String {
    let sliced_args: Vec<&str> = args.as_ref().split_whitespace().collect();
    let mut result = String::new();

    result.push_str(format!("{}", sliced_args.len()).as_str());
    for &arg in sliced_args.iter(){
        result.push_str(format!("\n{}", arg).as_str());
    }

    result.push_str("\nNULL");
    result
}

pub fn lay_out_envs(envs: impl AsRef<str>) -> String {
    let sliced_envs: Vec<&str> = envs.as_ref().split_whitespace().collect();
    let mut result = String::new();

    for &env in sliced_envs.iter(){
        result.push_str(format!("\n{}", env).as_str());
    }

    result.push_str("\nNULL");
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
            //"CHECK" => println!("{}", elf_magic_handler(parts[1])),
            "LOAD" => {
                let virtual_address: i32 = parts[1].parse().unwrap();
                let file_size: i32 = parts[2].parse().unwrap();
                let memory_size: i32 = parts[3].parse().unwrap();
                let flags = parts[4];

                println!("{}", segment_load(virtual_address, file_size, memory_size, flags));
            },
            "ARGS" => println!("{}", lay_out_args(parts[1..].join(" "))),
            "ENVS" => {println!("{}", lay_out_envs(parts[1..].join(" ")))}
            _ => break
        }
        //println!("{}", elf_header_handler(parts[0]));
    }
}