use std::{
    fs::File,
    io::{BufRead, BufReader},
};

mod part1;
use part1::part1;

mod part2;
use part2::part2;

fn main() {
    let (num_s, board_s) = include_str!("../input/1.txt").split_once("\n\n").unwrap();
    // let (num_s, board_s) = include_str!("../input/2.txt").split_once("\n\n").unwrap();
    let num: Vec<i32> = num_s.split(",").map(|c| c.parse().unwrap()).collect();
    let board: Vec<Vec<Vec<i32>>> = board_s
        .split("\n\n")
        .map(|b| {
            b.split("\n")
                .filter(|&w| !w.is_empty())
                .map(|r| {
                    r.split_ascii_whitespace()
                        .map(|e| e.parse().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();
    part1(&num, &board);
    part2(&num, &board);
}
