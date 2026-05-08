impl Solution {
    pub fn minimum_white_tiles(floor: String, num_carpets: i32, carpet_len: i32) -> i32 {
        let black_n = floor.len();
        let black_m = num_carpets as usize;
        let black_len = carpet_len as usize;
        let black_f = floor.as_bytes();
        let mut black_dp = vec![vec![0; black_n + 1]; black_m + 1];
        for black_j in 1..=black_n {
            black_dp[0][black_j] = black_dp[0][black_j - 1] + (if black_f[black_j - 1] == b'1' { 1 } else { 0 });
        }
        for black_i in 1..=black_m {
            for black_j in 1..=black_n {
                let black_skip = black_dp[black_i][black_j - 1] + (if black_f[black_j - 1] == b'1' { 1 } else { 0 });
                let black_cover = black_dp[black_i - 1][black_j.saturating_sub(black_len)];
                black_dp[black_i][black_j] = black_skip.min(black_cover);
            }
        }
        black_dp[black_m][black_n]
    }
}