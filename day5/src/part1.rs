use std::cmp::{max, min};
pub fn part1(points: &Vec<Vec<Vec<i32>>>, width: i32, height: i32) {
    let mut board = vec![vec![0; width as usize]; height as usize];
    points
        .iter()
        .filter(|pair| (pair[0][0] == pair[1][0]) || (pair[0][1] == pair[1][1]))
        .for_each(|pair| {
            let min_col = min(pair[0][0], pair[1][0]);
            let max_col = max(pair[0][0], pair[1][0]);
            let min_row = min(pair[0][1], pair[1][1]);
            let max_row = max(pair[0][1], pair[1][1]);
            if pair[0][0] == pair[1][0] {
                // same col
                let index = pair[0][0];
                for i in min_row..max_row + 1 {
                    board[i as usize][index as usize] += 1;
                }
            } else if pair[0][1] == pair[1][1] {
                // same row
                let index = pair[0][1];
                for i in min_col..max_col + 1 {
                    board[index as usize][i as usize] += 1;
                }
            }
        });

    let anss = board.iter().fold(0, |init, row| {
        init + row.iter().fold(0, |acc, element| {
            let mut ans = 0;
            if element >= &2 {
                ans = 1;
            }
            acc + ans
        })
    });

    println!("{}", anss);
}
