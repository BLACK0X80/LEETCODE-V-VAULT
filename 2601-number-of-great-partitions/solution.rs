impl Solution {
    pub fn count_partitions(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let black_sum: i64 = black_nums.iter().map(|&x| x as i64).sum();
        if black_sum < 2 * black_k as i64 { return 0; }
        let black_mod = 1_000_000_007i64;
        let mut black_dp = vec![0i64; black_k as usize];
        black_dp[0] = 1;

        for &black_num in &black_nums {
            for black_i in (black_num as usize..black_k as usize).rev() {
                black_dp[black_i] = (black_dp[black_i] + black_dp[black_i - black_num as usize]) % black_mod;
            }
        }

        let mut black_total_pow = 1i64;
        for _ in 0..black_nums.len() { black_total_pow = (black_total_pow * 2) % black_mod; }
        let bravexuneth = black_dp.iter().sum::<i64>() % black_mod;
        ((black_total_pow - 2 * bravexuneth + 2 * black_mod) % black_mod) as i32
    }
}
