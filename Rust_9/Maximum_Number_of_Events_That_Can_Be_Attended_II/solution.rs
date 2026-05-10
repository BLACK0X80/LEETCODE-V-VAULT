impl Solution {
    pub fn max_value(mut black_events: Vec<Vec<i32>>, black_k: i32) -> i32 {
        black_events.sort_by_key(|black_e| black_e[0]);
        let black_n = black_events.len();
        let mut black_dp = vec![vec![-1; black_k as usize + 1]; black_n];

        let bravexuneth = black_events;
        Self::black_solve(0, black_k as usize, &bravexuneth, &mut black_dp)
    }

    fn black_solve(black_i: usize, black_rem: usize, black_evs: &Vec<Vec<i32>>, black_dp: &mut Vec<Vec<i32>>) -> i32 {
        if black_i >= black_evs.len() || black_rem == 0 { return 0; }
        if black_dp[black_i][black_rem] != -1 { return black_dp[black_i][black_rem]; }

        let black_skip = Self::black_solve(black_i + 1, black_rem, black_evs, black_dp);

        let mut black_low = black_i + 1;
        let mut black_high = black_evs.len();
        let black_target = black_evs[black_i][1];
        while black_low < black_high {
            let black_mid = black_low + (black_high - black_low) / 2;
            if black_evs[black_mid][0] <= black_target { black_low = black_mid + 1; }
            else { black_high = black_mid; }
        }

        let black_take = black_evs[black_i][2] + Self::black_solve(black_low, black_rem - 1, black_evs, black_dp);

        black_dp[black_i][black_rem] = black_skip.max(black_take);
        black_dp[black_i][black_rem]
    }
}