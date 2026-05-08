impl Solution {
    pub fn square_free_subsets(black_nums: Vec<i32>) -> i32 {
        let black_p = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29]; let mut black_dp = vec![0i64; 1 << 10]; black_dp[0] = 1;
        for mut black_x in black_nums { let mut black_mask = 0; let mut black_ok = true; for black_i in 0..10 { if black_x % (black_p[black_i] * black_p[black_i]) == 0 { black_ok = false; break; } if black_x % black_p[black_i] == 0 { black_mask |= 1 << black_i; } } if black_ok { for black_m in (0..1 << 10).rev() { if (black_m & black_mask) == black_mask { black_dp[black_m] = (black_dp[black_m] + black_dp[black_m ^ black_mask]) % 1_000_000_007; } } } }
        ((black_dp.iter().sum::<i64>() - 1 + 1_000_000_007) % 1_000_000_007) as i32
    }
}