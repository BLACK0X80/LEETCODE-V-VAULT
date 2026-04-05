impl Solution {
    pub fn max_size_slices(slices: Vec<i32>) -> i32 {
        let black_m = slices.len();
        let black_n = black_m / 3;

        fn black_solve(black_arr: &[i32], black_count: usize) -> i32 {
            let black_len = black_arr.len();
            let mut black_dp = vec![vec![0; black_count + 1]; black_len + 1];

            for black_i in 1..=black_len {
                for black_j in 1..=black_count {
                    if black_i == 1 {
                        black_dp[black_i][black_j] = black_arr[0];
                    } else {
                        black_dp[black_i][black_j] = black_dp[black_i - 1][black_j].max(
                            black_dp[black_i - 2][black_j - 1] + black_arr[black_i - 1]
                        );
                    }
                }
            }
            black_dp[black_len][black_count]
        }

        let black_case1 = black_solve(&slices[0..black_m - 1], black_n);
        let black_case2 = black_solve(&slices[1..black_m], black_n);

        black_case1.max(black_case2)
    }
}
