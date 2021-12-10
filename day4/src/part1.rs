pub fn part1(nums: &Vec<i32>, board: &Vec<Vec<Vec<i32>>>) {
    let mut check: Vec<Vec<Vec<bool>>> =
        vec![vec![vec![true; board[0][0].len()]; board[0].len()]; board.len()];
    for n in nums {
        for i in 0..board.len() {
            check[i] = check_in_board(&board[i], &check[i], n);
            let (ok, score) = board_success(&board[i], &check[i]);
            if ok {
                println!("{}, {}, {}", score * n, score, n);
                return;
            }
        }
    }
}

fn check_in_board(board: &Vec<Vec<i32>>, state: &Vec<Vec<bool>>, target: &i32) -> Vec<Vec<bool>> {
    let mut s = state.clone();

    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == *target {
                s[i][j] = false;
            }
        }
    }

    s
}

fn board_success(b: &Vec<Vec<i32>>, state: &Vec<Vec<bool>>) -> (bool, i32) {
    let mut ans: bool = false;
    // Each row
    for i in 0..b.len() {
        let mut success: bool = true;
        for j in 0..b[i].len() {
            success &= !state[i][j];
        }
        ans |= success;
    }

    // Each column
    for j in 0..b[0].len() {
        let mut success: bool = true;
        for i in 0..b.len() {
            success &= !state[i][j];
        }
        ans |= success;
    }

    let mut score: i32 = 0;
    if ans {
        for i in 0..b.len() {
            for j in 0..b[0].len() {
                if state[i][j] {
                    score += b[i][j];
                }
            }
        }
    }

    (ans, score)
}
