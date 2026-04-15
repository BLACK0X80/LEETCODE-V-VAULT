impl Solution {
    pub fn num_ways(steps: i32, arr_len: i32) -> i32 {
        const M: u64 = 1_000_000_007;
        let max_pos = ((steps / 2) + 1).min(arr_len) as usize;
        let mut dp = vec![0u64; max_pos + 2];
        dp[0] = 1;

        for _ in 0..steps {
            let mut ndp = vec![0u64; max_pos + 2];
            for i in 0..max_pos {
                if dp[i] == 0 { continue; }
                ndp[i] += dp[i];
                ndp[i+1] += dp[i];
                if i > 0 { ndp[i-1] += dp[i]; }
            }
            for i in 0..max_pos { ndp[i] %= M; }
            dp = ndp;
        }

        dp[0] as i32
    }
}