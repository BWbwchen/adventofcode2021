use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod pair;
use pair::Pair;

mod part1;
use part1::part1;
mod part2;
use part2::part2;

fn main() {
    let filename = "input/1.txt";

    let commands = read_input_to_vec(filename);
    part1(&commands);
    part2(&commands);
}

fn read_input_to_vec(filename: &str) -> Vec<Pair> {
    let file = File::open(filename).unwrap();
    let content = BufReader::new(file);
    let mut vec: Vec<Pair> = Vec::new();
    for line in content.lines() {
        let s = line.unwrap();
        let mut p_iter = s.split_whitespace();

        vec.push(Pair {
            operator: String::from(p_iter.next().unwrap()),
            shift: p_iter.next().unwrap().parse().unwrap(),
        })
    }
    vec
}
