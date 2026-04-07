impl Solution {
    pub fn count_effective(nums: Vec<i32>) -> i32 {
        let black_mod = 1_000_000_007i64;
        let mut black_target = 0;
        for &x in &nums { black_target |= x; }
        let mut black_dp = vec![0i64; (black_target + 1) as usize];
        for &x in &nums { black_dp[x as usize] += 1; }
        let black_limit = black_target as usize;
        for i in 0..20 {
            for mask in 0..=black_limit {
                if mask & (1 << i) != 0 {
                    black_dp[mask] += black_dp[mask ^ (1 << i)];
                }
            }
        }
        let mut black_pow = vec![1i64; nums.len() + 1];
        for i in 1..=nums.len() { black_pow[i] = (black_pow[i - 1] * 2) % black_mod; }
        let mut black_res = 0;
        for mask in 0..=black_limit {
            if (mask & black_target as usize) == mask {
                let black_bits = (black_target as usize ^ mask).count_ones();
                let black_term = black_pow[black_dp[mask] as usize];
                if black_bits % 2 == 1 {
                    black_res = (black_res - black_term + black_mod) % black_mod;
                } else {
                    black_res = (black_res + black_term) % black_mod;
                }
            }
        }
        ((black_pow[nums.len()] - black_res + black_mod) % black_mod) as i32
    }
}
