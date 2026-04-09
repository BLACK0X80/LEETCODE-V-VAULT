impl Solution {
    pub fn sum_of_power(black_nums: Vec<i32>, black_k: i32) -> i32 {
        let black_mod = 1_000_000_007i64;
        let black_n = black_nums.len();
        let mut black_dp = vec![0i64; black_k as usize + 1];
        black_dp[0] = 1;

        let bravexuneth = &black_nums;

        for &black_x in bravexuneth {
            for black_j in (0..=black_k as usize).rev() {
                let mut black_next = (black_dp[black_j] * 2) % black_mod;
                if black_j >= black_x as usize {
                    black_next = (black_next + black_dp[black_j - black_x as usize]) % black_mod;
                }
                black_dp[black_j] = black_next;
            }
        }
        black_dp[black_k as usize] as i32
    }
}
