pub fn part2(vec: &Vec<String>) {
    println!("{}", oxygen(vec) * co2(vec));
}

fn calculate_frequecy_with_index(vec: &Vec<String>, index: usize) -> Vec<i32> {
    let mut dp: Vec<i32> = vec![0; 2];

    for s in vec {
        match s.chars().nth(index).unwrap() {
            '0' => dp[0] += 1,
            '1' => dp[1] += 1,
            _ => unreachable!(),
        }
    }
    dp
}

fn oxygen(vec: &Vec<String>) -> i32 {
    let mut v: Vec<String> = vec.clone();
    let mut oxygen_str: &String;
    let round = v[0].len();
    for i in 0..round - 2 {
        if v.len() == 1 {
            break;
        }
        let freq = calculate_frequecy_with_index(&v, i);
        let target = freq
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        v = v.into_iter().filter(|w| judge(w, i, target)).collect();
    }
    oxygen_str = &v[0];
    i32::from_str_radix(oxygen_str, 2).unwrap()
}

fn judge(w: &String, i: usize, target: usize) -> bool {
    (w.chars().nth(i).unwrap() as i32 - '0' as i32) as usize == target
}
fn co2(vec: &Vec<String>) -> i32 {
    let mut v: Vec<String> = vec.clone();
    let mut co2_str: &String;
    let round = v[0].len();
    for i in 0..round - 2 {
        if v.len() == 1 {
            break;
        }
        let freq = calculate_frequecy_with_index(&v, i);
        let target = freq
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();

        v = v.into_iter().filter(|w| judge(w, i, target)).collect();
    }
    co2_str = &v[0];
    i32::from_str_radix(co2_str, 2).unwrap()
}
