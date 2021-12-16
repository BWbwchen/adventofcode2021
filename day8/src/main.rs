fn main() {
    part1();
    part2();
}

fn part1() {
    let _input = include_str!("../input/1.txt")
        .lines()
        .flat_map(|s| s.split_once("|").unwrap().1.split_ascii_whitespace())
        .filter(|s| matches!(s.len(), 2 | 3 | 4 | 7))
        .count();
    println!("{:?}", _input);
}

fn part2() {
    let ans_l = include_str!("../input/1.txt").lines().map(|l| {
        let mut whole = l.split("|");
        let mut first = whole.next().unwrap().split(" ").filter(|w| !w.is_empty());
        let one = first.clone().find(|w| w.len() == 2).unwrap();
        let four = first.clone().find(|w| w.len() == 4).unwrap();

        let ans = whole
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .map(|w| match w.len() {
                2 => 1,
                4 => 4,
                3 => 7,
                7 => 8,
                len => match (
                    len,
                    w.chars().into_iter().filter(|&ww| one.contains(ww)).count(),
                    w.chars()
                        .into_iter()
                        .filter(|&ww| four.contains(ww))
                        .count(),
                ) {
                    (6, 2, 3) => 0,
                    (5, 1, 2) => 2,
                    (5, 2, 3) => 3,
                    (5, 1, 3) => 5,
                    (6, 1, 3) => 6,
                    (6, 2, 4) => 9,
                    _ => unreachable!(),
                },
            })
            .fold(0, |acc, n| 10 * acc + n);
        ans
    });

    println!("{}", ans_l.sum::<u32>());
}
