pub fn part1(vec: &Vec<String>) {
    let mut dp: Vec<Vec<i32>> = vec![vec![0; 2]; vec[0].len()];

    for s in vec {
        for (i, c) in s.chars().enumerate() {
            match c {
                '0' => dp[i][0] += 1,
                '1' => dp[i][1] += 1,
                _ => unreachable!(),
            }
        }
    }

    let mut gamma_str: String = String::from("");
    let mut epsilon_str: String = String::from("");

    for v in dp {
        if v[0] > v[1] {
            gamma_str.push('0');
            epsilon_str.push('1');
        } else {
            gamma_str.push('1');
            epsilon_str.push('0');
        }
    }

    let gamma = isize::from_str_radix(&gamma_str[..], 2).unwrap();
    let epsilon = isize::from_str_radix(&epsilon_str[..], 2).unwrap();

    println!("{}", gamma * epsilon)
}
