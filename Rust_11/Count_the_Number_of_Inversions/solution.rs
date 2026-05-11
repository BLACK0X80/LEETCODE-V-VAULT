impl Solution {
    pub fn number_of_permutations(n: i32, requirements: Vec<Vec<i32>>) -> i32 {
        let black_mod = 1_000_000_007;
        let mut black_req = vec![-1; n as usize];
        for r in requirements { black_req[r[0] as usize] = r[1]; }
        if black_req[0] > 0 { return 0; }
        let mut black_dp = vec![0i64; 401];
        black_dp[0] = 1;
        for i in 1..n as usize {
            let mut black_next = vec![0i64; 401];
            let mut black_sum = 0i64;
            for j in 0..401 {
                black_sum = (black_sum + black_dp[j]) % black_mod;
                if j > i { black_sum = (black_sum - black_dp[j - i - 1] + black_mod) % black_mod; }
                if black_req[i] == -1 || black_req[i] == j as i32 { black_next[j] = black_sum; }
            }
            black_dp = black_next;
        }
        black_dp[black_req[n as usize - 1] as usize] as i32
    }
}