use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let filename = "input/1.txt";

    let content = read_input_to_vec(filename);
    part1(&content);
    part2(&content);
}

fn read_input_to_vec(filename: &str) -> Vec<i32> {
    let file = File::open(filename).unwrap();
    let content = BufReader::new(file);
    let mut vec: Vec<i32> = Vec::new();
    for line in content.lines() {
        vec.push(line.unwrap().parse().unwrap())
    }
    vec
}

fn part1(vec: &Vec<i32>) {
    let mut now = i32::MAX;
    let mut count: i32 = 0;

    for i in 0..vec.len() as usize {
        let &num = vec.get(i).unwrap();

        if now < num {
            count += 1;
        }

        now = num;
    }

    println!("{}", count);
}

fn part2(vec: &Vec<i32>) {
    if vec.len() < 3 {
        println!("0")
    }
    let mut cummulate: Vec<i32> = Vec::new();
    for i in 0..vec.len() - 2 as usize {
        cummulate.push((i..i + 3).fold(0, |sum, id| sum + vec.get(id).unwrap()) as i32)
    }
    part1(&cummulate);
}
