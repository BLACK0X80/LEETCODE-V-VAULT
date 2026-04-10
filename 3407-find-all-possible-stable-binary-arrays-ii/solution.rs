impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let black_mod = 1_000_000_007;
        let (z, o, l) = (zero as usize, one as usize, limit as usize);
        let mut black_dp0 = vec![vec![0i64; o + 1]; z + 1];
        let mut black_dp1 = vec![vec![0i64; o + 1]; z + 1];

        for i in 1..=z.min(l) { black_dp0[i][0] = 1; }
        for j in 1..=o.min(l) { black_dp1[0][j] = 1; }

        for i in 1..=z {
            for j in 1..=o {
                black_dp0[i][j] = (black_dp0[i - 1][j] + black_dp1[i - 1][j]) % black_mod;
                if i > l {
                    black_dp0[i][j] = (black_dp0[i][j] - black_dp1[i - l - 1][j] + black_mod) % black_mod;
                }
                black_dp1[i][j] = (black_dp1[i][j - 1] + black_dp0[i][j - 1]) % black_mod;
                if j > l {
                    black_dp1[i][j] = (black_dp1[i][j] - black_dp0[i][j - l - 1] + black_mod) % black_mod;
                }
            }
        }
        ((black_dp0[z][o] + black_dp1[z][o]) % black_mod) as i32
    }
}
