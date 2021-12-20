const NEXT: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];
fn main() {
    let mut input = include_str!("../input/1.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|w| w.to_digit(10).unwrap() as i32)
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<_>>();

    // println!("{:?}", input);
    part1(&mut input.clone(), 100);
    part2(&mut input.clone());
}

fn part1(m: &mut Vec<Vec<i32>>, days: usize) {
    let mut ans: u64 = 0;
    for _ in 0..days {
        ans += grow(m);
    }
    println!("{}", ans);
}

fn part2(m: &mut Vec<Vec<i32>>) {
    let total_element: u64 = (m.len() * m[0].len()) as u64;
    let mut i: u64 = 0;
    loop {
        i += 1;
        if grow(m) == total_element {
            println!("{}", i);
            break;
        }
    }
}

fn grow(m: &mut Vec<Vec<i32>>) -> u64 {
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            m[i][j] += 1;
        }
    }

    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] > 9 {
                update(m, i, j);
            }
        }
    }

    let mut zero: u64 = 0;
    for i in 0..m.len() {
        for j in 0..m[0].len() {
            if m[i][j] == 0 {
                zero += 1;
            }
        }
    }

    zero
}

fn update(m: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    if m[i][j] <= 9 {
        return;
    }
    m[i][j] = 0;
    for delta in NEXT {
        if valid(m, i as i32, j as i32, delta.0, delta.1) {
            if m[(i as i32 + delta.0) as usize][(j as i32 + delta.1) as usize] != 0 {
                m[(i as i32 + delta.0) as usize][(j as i32 + delta.1) as usize] += 1;
                if m[(i as i32 + delta.0) as usize][(j as i32 + delta.1) as usize] > 9 {
                    update(
                        m,
                        (i as i32 + delta.0) as usize,
                        (j as i32 + delta.1) as usize,
                    );
                }
            }
        }
    }
}

fn valid(m: &Vec<Vec<i32>>, row: i32, col: i32, row_delta: i32, col_delta: i32) -> bool {
    0 <= row + row_delta
        && row + row_delta < m.len() as i32
        && 0 <= col + col_delta
        && col + col_delta < m[0].len() as i32
}
