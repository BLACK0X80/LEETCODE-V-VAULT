use std::collections::HashMap;

impl Solution {
    pub fn count_sub_multisets(black_nums: Vec<i32>, black_l: i32, black_r: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_counts = HashMap::new();
        for black_n in black_nums { *black_counts.entry(black_n).or_insert(0) += 1; }

        let black_zeros = *black_counts.get(&0).unwrap_or(&0);
        let mut black_dp = vec![0i64; black_r as usize + 1];
        black_dp[0] = 1;

        for (&black_val, &black_occ) in black_counts.iter() {
            if black_val == 0 { continue; }
            let black_v = black_val as usize;
            for i in black_v..=black_r as usize {
                black_dp[i] = (black_dp[i] + black_dp[i - black_v]) % black_mod;
            }
            let black_limit = (black_occ + 1) as usize * black_v;
            for i in (black_limit..=black_r as usize).rev() {
                black_dp[i] = (black_dp[i] - black_dp[i - black_limit] + black_mod) % black_mod;
            }
        }

        let mut black_res = 0;
        for i in black_l as usize..=black_r as usize {
            black_res = (black_res + black_dp[i]) % black_mod;
        }
        ((black_res * (black_zeros as i64 + 1)) % black_mod) as i32
    }
}
