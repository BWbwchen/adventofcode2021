use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod part1;
use part1::part1;

mod part2;
use part2::part2;

fn main() {
    let filename = "input/1.txt";

    let commands = read_input_to_vec(filename);
    // part1(&commands);
    part2(&commands);
}

fn read_input_to_vec(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let content = BufReader::new(file);
    let mut vec: Vec<String> = Vec::new();
    for line in content.lines() {
        vec.push(line.unwrap())
    }
    vec
}
