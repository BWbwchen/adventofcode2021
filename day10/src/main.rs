use std::char;

fn main() {
    let input = include_str!("../input/1.txt").lines().collect::<Vec<_>>();
    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<&str>) {
    let mut ans: u64 = 0;
    let mut stack: Vec<char> = Vec::new();

    for i in 0..input.len() {
        for (_, ch) in input[i].chars().enumerate() {
            let delta = match ch {
                '<' | '(' | '{' | '[' => {
                    stack.push(ch);
                    0
                }
                '>' => {
                    if stack.last().unwrap() != &'<' {
                        reward('>')
                    } else {
                        stack.pop();
                        0
                    }
                }
                ')' => {
                    if stack.last().unwrap() != &'(' {
                        reward(')')
                    } else {
                        stack.pop();
                        0
                    }
                }
                '}' => {
                    if stack.last().unwrap() != &'{' {
                        reward('}')
                    } else {
                        stack.pop();
                        0
                    }
                }
                ']' => {
                    if stack.last().unwrap() != &'[' {
                        reward(']')
                    } else {
                        stack.pop();
                        0
                    }
                }
                _ => 0,
            };

            if delta != 0 {
                ans += delta;
                break;
            }
        }
    }

    println!("{:?}", ans);
}

fn part2(input: &Vec<&str>) {
    let mut ans_vec: Vec<u64> = Vec::new();
    let mut stack: Vec<char> = Vec::new();

    for i in 0..input.len() {
        let mut temp_ans: u64 = 0;
        let mut bad: bool = false;
        for (_, ch) in input[i].chars().enumerate() {
            let delta = match ch {
                '<' | '(' | '{' | '[' => {
                    stack.push(ch);
                    0
                }
                '>' => {
                    if stack.last().unwrap() != &'<' {
                        reward('>')
                    } else {
                        stack.pop();
                        0
                    }
                }
                ')' => {
                    if stack.last().unwrap() != &'(' {
                        reward(')')
                    } else {
                        stack.pop();
                        0
                    }
                }
                '}' => {
                    if stack.last().unwrap() != &'{' {
                        reward('}')
                    } else {
                        stack.pop();
                        0
                    }
                }
                ']' => {
                    if stack.last().unwrap() != &'[' {
                        reward(']')
                    } else {
                        stack.pop();
                        0
                    }
                }
                _ => 0,
            };

            if delta != 0 {
                bad = true;
                break;
            }
        }

        if !bad && stack.len() != 0 {
            for (_, &item) in stack.iter().rev().enumerate() {
                temp_ans *= 5 as u64;
                temp_ans += reward_addition(item);
            }
            ans_vec.push(temp_ans);
        }
        stack.clear();
    }

    ans_vec.sort();

    println!("{:?}", ans_vec[ans_vec.len() / 2]);
    // println!("{:?}", ans_vec);
}

fn reward_addition(ch: char) -> u64 {
    match ch {
        '(' | ')' => 1,
        '[' | ']' => 2,
        '{' | '}' => 3,
        '<' | '>' => 4,
        _ => 0,
    }
}

fn reward(ch: char) -> u64 {
    match ch {
        '(' | ')' => 3,
        '[' | ']' => 57,
        '{' | '}' => 1197,
        '<' | '>' => 25137,
        _ => 0,
    }
}
