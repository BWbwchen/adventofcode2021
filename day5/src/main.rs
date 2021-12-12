use std::cmp::max;

mod part1;
use part1::part1;
mod part2;
use part2::part2;

fn main() {
    let points = include_str!("../input/1.txt")
        .split('\n')
        .filter(|w| !w.is_empty())
        .map(|line| {
            line.split("->")
                .map(|pair| {
                    pair.replace(" ", "")
                        .split(",")
                        .map(|p| p.parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let (width, height) = points.iter().fold((0, 0), |(w, h), point_pair| {
        let (ww, hh) = point_pair.iter().fold((w, h), |(ww, hh), point| {
            (max(ww, point[0]), max(hh, point[1]))
        });
        (max(ww, w), max(hh, h))
    });
    part1(&points, width + 1, height + 1);
    part2(&points, width + 1, height + 1);
}
