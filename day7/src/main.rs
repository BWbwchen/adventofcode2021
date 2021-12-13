use std::cmp::min;
fn main() {
    let inputs = include_str!("../input/1.txt")
        .split("\n")
        .filter(|w| !w.is_empty())
        .collect::<String>();

    let input = inputs
        .split(",")
        .map(|n| n.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // println!("{:?}", input);
    part1(&input);
    part2(&input);
}

fn part1(points: &Vec<i32>) {
    let &minn: &i32 = points.iter().min().unwrap();
    let &maxn: &i32 = points.iter().max().unwrap();

    let ans: i32 = (minn..maxn + 1).fold(i32::MAX, |acc, position| {
        min(
            acc,
            points
                .iter()
                .fold(0, |sum, fuel| sum + (fuel - position).abs()),
        )
    });

    println!("{}, {}, {}", ans, minn, maxn);
}

fn part2(points: &Vec<i32>) {
    let &minn: &i32 = points.iter().min().unwrap();
    let &maxn: &i32 = points.iter().max().unwrap();

    let ans: i32 = (minn..maxn + 1).fold(i32::MAX, |acc, position| {
        min(
            acc,
            points.iter().fold(0, |sum, fuel| {
                let n = (fuel - position).abs();
                sum + (n + 1) * n / 2
            }),
        )
    });

    println!("{}, {}, {}", ans, minn, maxn);
}
