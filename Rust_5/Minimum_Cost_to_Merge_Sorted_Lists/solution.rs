impl Solution {
    pub fn min_merge_cost(lists: Vec<Vec<i32>>) -> i64 {
        let black_n = lists.len();
        let black_m = 1 << black_n;
        let mut black_dp = vec![i64::MAX / 2; black_m];
        let mut black_lens = vec![0i64; black_m];
        let mut black_meds = vec![0i64; black_m];

        for i in 1..black_m {
            let mut black_v: Vec<i32> = Vec::new(); 
            for j in 0..black_n {
                if (i >> j) & 1 == 1 {
                    black_v.extend(&lists[j]);
                }
            }
            black_v.sort_unstable();
            black_lens[i] = black_v.len() as i64;
            black_meds[i] = black_v[(black_v.len() - 1) / 2] as i64;
        }

        for i in 0..black_n {
            black_dp[1 << i] = 0;
        }

        for i in 1..black_m {
            if i.count_ones() < 2 { continue; }
            let mut black_sub = (i - 1) & i;
            while black_sub > 0 {
                let black_l = black_sub;
                let black_r = i ^ black_sub;
                let black_c = black_lens[i] + (black_meds[black_l] - black_meds[black_r]).abs();
                black_dp[i] = black_dp[i].min(black_dp[black_l] + black_dp[black_r] + black_c);
                black_sub = (black_sub - 1) & i;
            }
        }
        black_dp[black_m - 1]
    }
}