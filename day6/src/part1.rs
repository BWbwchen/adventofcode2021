pub fn part1(e: &Vec<u64>, day: i32) {
    let mut dp = vec![0_u64; 9];

    e.iter().for_each(|&n| dp[n as usize] += 1);

    (0..day).for_each(|_| {
        dp.rotate_left(1);
        dp[6] += dp[8];
    });

    let ans = dp.iter().sum::<u64>();

    println!("{}", ans);
}
