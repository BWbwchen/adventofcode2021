mod part1;
use part1::part1;

fn main() {
    let s_input = include_str!("../input/1.txt")
        .split("\n")
        .filter(|&n| !n.is_empty())
        .collect::<String>();
    let input = s_input
        .split(",")
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    part1(&input, 80);
    part1(&input, 256);
}
