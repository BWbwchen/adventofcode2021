fn main() {
    let input = include_str!("../input/1.txt")
        // let input = include_str!("../input/2.txt")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    part1(&input);
    part2(&input);
}

fn part2(m: &Vec<Vec<u32>>) {
    // dfs
    let mut done: Vec<Vec<bool>> = vec![vec![false; m[0].len()]; m.len()];

    let mut ans_vec: Vec<i32> = Vec::new();

    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] == 9 {
                done[row][col] = true;
            }
        }
    }

    for row in 0..m.len() {
        for col in 0..m[0].len() {
            if m[row][col] != 9 && !done[row][col] {
                // need to dfs
                ans_vec.push(dfs(m, &mut done, row, col));
            } else if m[row][col] == 9 {
                done[row][col] = true;
            }
        }
    }

    ans_vec.sort_by(|a, b| b.cmp(a));

    println!("{:?}", ans_vec[0] * ans_vec[1] * ans_vec[2]);
}

fn dfs(m: &Vec<Vec<u32>>, done: &mut Vec<Vec<bool>>, row: usize, col: usize) -> i32 {
    let mut ans = 1;

    done[row][col] = true;
    // up
    if row >= 1 && !done[row - 1][col] {
        ans += dfs(m, done, row - 1, col);
    }

    // down
    if (row + 1) < m.len() && !done[row + 1][col] {
        ans += dfs(m, done, row + 1, col);
    }

    // right
    if (col + 1) < m[0].len() && !done[row][col + 1] {
        ans += dfs(m, done, row, col + 1);
    }

    // left
    if col >= 1 && !done[row][col - 1] {
        ans += dfs(m, done, row, col - 1);
    }

    ans
}

fn part1(m: &Vec<Vec<u32>>) {
    let mut ans = 0;
    for i in 0..m.len() {
        for j in 0..m[i].len() {
            if valid(i, j, m) {
                ans += m[i][j] + 1;
            }
        }
    }
    println!("{:?}", ans);
}

fn valid(i: usize, j: usize, m: &Vec<Vec<u32>>) -> bool {
    let mut ok = true;

    // up
    if i >= 1 {
        ok &= m[i][j] < m[i - 1][j];
    }

    // down
    if (i + 1) < m.len() {
        ok &= m[i][j] < m[i + 1][j];
    }

    // right
    if (j + 1) < m[0].len() {
        ok &= m[i][j] < m[i][j + 1];
    }

    // left
    if j >= 1 {
        ok &= m[i][j] < m[i][j - 1];
    }

    ok
}
