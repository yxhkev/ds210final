use std::io;
use std::fs::File;
use std::io::prelude::*;


fn main() {


    let mut graph = Vec::new();

    let mut f = match File::open("data.txt") {
        Ok(file) => file,
        Err(_) => panic!("Failed to open the file"),
    };

    let mut line = String::new();
    while f.read_line(&mut line).unwrap() > 0 {
        let line_parts: Vec<&str> = line.trim().split_whitespace().collect();

        let edge: (u32, u32) = (line_parts[0].parse().unwrap(), line_parts[1].parse().unwrap());

        graph.push(edge);

        line.clear();
    }
